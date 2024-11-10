<script setup lang="ts">
import { computed, ref, watch } from 'vue'

import sentenceAudio from '../assets/sentence-original.wav'
import wasmUrl from '../wasm_realtime_sws/wasm_audio_bg.wasm?url'
import {
  getAudioBuffer,
  getWebAudioMediaStream,
  trimAudioBufferToMultipleOf,
} from './audioUtils.ts'
import LiveVisualization from './components/LiveVisualization.vue'
import PlaybackControls from './components/PlaybackControls.vue'
import Slider from './components/Slider.vue'
import SineWaveSpeechNode from './sineWaveSpeechNode.ts'
// Importing with "?worker&url" and not "?url" is necessary:
// https://github.com/vitejs/vite/issues/6979#issuecomment-1320394505
import processorUrl from './sineWaveSpeechProcessor.ts?worker&url'
import { Hop, PlaybackState } from './types.ts'

// See BLOCK_SIZE in sineWaveSpeechProcessor.ts.
// We can't import that here because it's a Worker (I think?).
const BLOCK_SIZE = 128

// The sample rate significantly affects how sine wave speech effect sounds.
// 8000 is the tested one.
const SAMPLE_RATE = 8000

// Single source of truth for the recording duration (in seconds)
const RECORDING_DURATION_BLOCKS = 128
const RECORDING_DURATION_SEC = (RECORDING_DURATION_BLOCKS * BLOCK_SIZE) / SAMPLE_RATE

const TOTAL_NUM_HOPS_WHEN_LIVE = 48

const hops = ref<Hop[]>([])
const recordedAudioBuffer = ref<AudioBuffer | null>(null)
const audioSourceNode = ref<MediaStreamAudioSourceNode | AudioBufferSourceNode | null>(
  null
)

// Synthesis parameters
const nWaves = ref(4)
const frequencyQuantizationStrength = ref(0.0)
const hopSizeMultiplier = ref(2)
const gainDb = ref(0)

const totalNumHops = computed(() => {
  const source = audioSourceNode.value
  if (source instanceof AudioBufferSourceNode) {
    // Pre-recorded source. we know how many hops there are.
    if (source.buffer === null) throw new Error('Buffer is null')
    const hopSize = BLOCK_SIZE * hopSizeMultiplier.value
    return Math.ceil(source.buffer.length / hopSize)
  } else if (source instanceof MediaStreamAudioSourceNode) {
    // Live source - a microphone.
    if (playbackState.value === PlaybackState.Recording) {
      return RECORDING_DURATION_BLOCKS / hopSizeMultiplier.value
    } else if (playbackState.value === PlaybackState.PlayingRealtime) {
      return TOTAL_NUM_HOPS_WHEN_LIVE
    } else if (playbackState.value === PlaybackState.Stopped) {
      // We get into this branch briefly when we stop real-time playback,
      // probably because this computed() is called before the state update is handled.
      return TOTAL_NUM_HOPS_WHEN_LIVE
    } else {
      throw new Error('Invalid playback state')
    }
  } else {
    return null
  }
})

type AudioSetup = {
  audioContext: AudioContext
  sineWaveSpeechNode: SineWaveSpeechNode
}

const setupAudio = async (): Promise<AudioSetup> => {
  const audioContext = new window.AudioContext({ sampleRate: SAMPLE_RATE })

  // Fetch the raw WebAssembly module
  const response = await window.fetch(wasmUrl)
  const wasmBytes = await response.arrayBuffer()

  // Add our audio processor worklet to the context.
  await audioContext.audioWorklet.addModule(processorUrl)

  // Create the AudioWorkletNode which enables the main thread to
  // communicate with the audio processor (which runs in a Worklet).
  let sineWaveSpeechNode = new SineWaveSpeechNode(
    audioContext,
    'sine-wave-speech-processor',
    wasmBytes,
    {
      processorOptions: {},
      onHop: (hop: Hop) => {
        if (totalNumHops.value != null) {
          // We're playing from a recording
          if (hops.value.length >= totalNumHops.value) {
            hops.value = []
          }
        } else {
          // We're playing from the microphone
          if (hops.value.length > TOTAL_NUM_HOPS_WHEN_LIVE) {
            hops.value = []
          }
        }
        hops.value.push(hop)
      },
    }
  )
  sineWaveSpeechNode.connect(audioContext.destination)

  return { audioContext, sineWaveSpeechNode }
}

const audioSetup = ref<AudioSetup | null>(null)

const getAudioSetup = async () => {
  if (!audioSetup.value) {
    audioSetup.value = await setupAudio()
  }
  return audioSetup.value
}

const updateParameter = async (parameterName: string, value: number) => {
  const { audioContext, sineWaveSpeechNode } = await getAudioSetup()
  const param = sineWaveSpeechNode.parameters.get(parameterName)
  if (param === undefined) throw new Error(`Parameter ${parameterName} not found`)

  param.setValueAtTime(value, audioContext.currentTime)
}

const setFrequencyQuantizationStrength = async (
  newFrequencyQuantizationStrength: number
) => {
  updateParameter('frequencyQuantizationStrength', newFrequencyQuantizationStrength)
}

const setHopSizeMultiplier = async (newHopSizeMultiplier: number) => {
  updateParameter('hopSizeMultiplier', newHopSizeMultiplier)
}

const setNWaves = async (newNWaves: number) => {
  updateParameter('nWaves', newNWaves)
}

const setGainDb = async (newGainDb: number) => {
  updateParameter('gainDb', newGainDb)
}

watch(frequencyQuantizationStrength, setFrequencyQuantizationStrength)
watch(hopSizeMultiplier, setHopSizeMultiplier)
watch(nWaves, setNWaves)
watch(gainDb, setGainDb)

const frequencyQuantizationName = computed(() => {
  const strength = frequencyQuantizationStrength.value
  const breakpoints = [
    { upTo: 0.5, name: 'Microtonal' },
    { upTo: 1.5, name: 'Chromatic' },
    { upTo: 2.5, name: 'Diatonic' },
    { upTo: 3.0, name: 'Pentatonic' },
  ]
  for (const { upTo, name } of breakpoints) {
    if (strength <= upTo) {
      return name
    }
  }

  throw new Error(`Invalid frequency quantization strength ${strength}`)
})

const playbackState = ref<PlaybackState>(PlaybackState.Stopped)

watch(playbackState, async (newPlaybackState: PlaybackState) => {
  const { audioContext } = await getAudioSetup()

  switch (newPlaybackState) {
    case PlaybackState.PlayingRecorded:
      if (audioSourceNode.value instanceof AudioBufferSourceNode) {
        // No need to restart, we were just paused
        audioContext.resume()
      } else {
        await startPlayingAudio(false)
      }
      break
    case PlaybackState.PlayingRealtime:
      await startPlayingAudio(true)
      break
    case PlaybackState.Recording:
      await startRecordingAudio()
      break
    case PlaybackState.Stopped:
      if (audioSourceNode.value instanceof MediaStreamAudioSourceNode) {
        // If we're playing from the microphone, get rid of that node - won't need it anymore.
        // If it was a recording, we can just suspend so that we can pick up where we left off later.
        await setAudioSourceNode(null)
      }
      audioContext.suspend()
      break
  }
})

/**
 * Set the audio source node to a new one, disconnecting the old one if it exists.
 */
const setAudioSourceNode = async (
  newValue: MediaStreamAudioSourceNode | AudioBufferSourceNode | null
) => {
  const oldValue = audioSourceNode.value
  if (oldValue !== null) {
    oldValue.disconnect()
    if (oldValue instanceof AudioBufferSourceNode) {
      oldValue.stop()
    }
  }

  audioSourceNode.value = newValue
  hops.value = []
}

const startPlayingAudio = async (fromMicrophone: boolean) => {
  const { audioContext, sineWaveSpeechNode } = await getAudioSetup()

  // The watch() calls above only happen when the value is updated, so if the user didn't change
  // anything, the parameters might be out of sync with the audio processor.
  setFrequencyQuantizationStrength(frequencyQuantizationStrength.value)
  setHopSizeMultiplier(hopSizeMultiplier.value)
  setNWaves(nWaves.value)
  setGainDb(gainDb.value)

  if (audioSourceNode.value !== null) {
    audioSourceNode.value.disconnect()
    if (audioSourceNode.value instanceof AudioBufferSourceNode) {
      audioSourceNode.value.stop()
    }
  }

  if (fromMicrophone) {
    const mediaStream = await getWebAudioMediaStream()
    const source = audioContext.createMediaStreamSource(mediaStream)
    source.connect(sineWaveSpeechNode)
    setAudioSourceNode(source)
  } else {
    const dryAudioBuffer =
      recordedAudioBuffer.value || (await getAudioBuffer(audioContext, sentenceAudio))

    const trimmedAudioBuffer = trimAudioBufferToMultipleOf(
      audioContext,
      dryAudioBuffer,
      BLOCK_SIZE * 4
    )

    let bufferSource = audioContext.createBufferSource()
    bufferSource.buffer = trimmedAudioBuffer
    bufferSource.loop = true
    bufferSource.connect(sineWaveSpeechNode)
    bufferSource.start()
    setAudioSourceNode(bufferSource)
  }

  hops.value = []
  await audioContext.resume()
}

const startRecordingAudio = async () => {
  const { audioContext, sineWaveSpeechNode } = await getAudioSetup()
  const mediaStream = await getWebAudioMediaStream()
  const mediaRecorder = new MediaRecorder(mediaStream)
  const audioChunks: BlobPart[] = []

  const source = audioContext.createMediaStreamSource(mediaStream)
  source.connect(sineWaveSpeechNode)

  mediaRecorder.ondataavailable = (event) => {
    audioChunks.push(event.data)
  }

  mediaRecorder.onstop = async () => {
    const audioBlob = new Blob(audioChunks)
    const arrayBuffer = await audioBlob.arrayBuffer()
    recordedAudioBuffer.value = await audioContext.decodeAudioData(arrayBuffer)
    startPlayingAudio(false)
    playbackState.value = PlaybackState.PlayingRecorded
    // Re-connect the sineWaveSpeechNode to the destination so that we can hear the sound again
    sineWaveSpeechNode.connect(audioContext.destination)
  }

  setAudioSourceNode(source)
  await audioContext.resume()
  // We want to show the visualization as we're recording, but not play (monitor) the audio
  // to avoid feedback if the user is using speakers.
  sineWaveSpeechNode.disconnect(audioContext.destination)
  // Start with an empty array of hops to make it clear it's separate from the playback
  hops.value = []

  mediaRecorder.start()

  setTimeout(() => {
    mediaRecorder.stop()
  }, RECORDING_DURATION_SEC * 1000)
}
</script>

<template>
  <div
    class="grid grid-cols-1 content-center justify-items-center h-screen font-body gap-2"
  >
    <h1 class="text-5xl italic font-header">Sine Wave Speech</h1>
    <PlaybackControls v-model="playbackState">
      <div
        class="mt-2 h-2 bg-white overflow-hidden rounded-sm w-full col-span-2"
        :style="{ '--recording-duration': `${RECORDING_DURATION_SEC}s` }"
      >
        <div
          class="progress-bar-animation bg-accent1 w-full h-full"
          v-if="playbackState === PlaybackState.Recording"
        ></div>
      </div>
    </PlaybackControls>

    <div class="mt-2">
      <Slider
        v-model="nWaves"
        :label="`Number of waves: ${nWaves}`"
        :min="1"
        :max="16"
        id="n-waves-slider"
      />
      <Slider
        v-model="frequencyQuantizationStrength"
        :label="`Scale: ${frequencyQuantizationName}`"
        :min="0"
        :max="3"
        :step="0.1"
        id="frequency-quantization-level-slider"
      />
      <Slider
        v-model="hopSizeMultiplier"
        :label="`Step size: ${hopSizeMultiplier}`"
        :min="1"
        :max="16"
        id="hop-size-multiplier-slider"
      />
      <Slider
        v-model="gainDb"
        :label="`Gain: ${gainDb} dB`"
        :min="-18"
        :max="18"
        id="gain-db-slider"
      />
    </div>

    <div class="max-w-3xl">
      <LiveVisualization
        :hops="hops"
        :totalNumHops="totalNumHops"
        :sampleRate="SAMPLE_RATE"
      />
    </div>
  </div>
</template>
<style lang="css" scoped>
.progress-bar-animation {
  animation: progress var(--recording-duration) linear;
  transform-origin: left;
}

@keyframes progress {
  0% {
    transform: scaleX(0);
  }
  100% {
    transform: scaleX(1);
  }
}
</style>
