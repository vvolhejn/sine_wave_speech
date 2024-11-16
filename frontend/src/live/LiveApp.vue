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
import SynthesisParameterControls from './components/ParameterControls.vue'
import PlaybackControls from './components/PlaybackControls.vue'
import SineWaveSpeechNode from './sineWaveSpeechNode.ts'
// Importing with "?worker&url" and not "?url" is necessary:
// https://github.com/vitejs/vite/issues/6979#issuecomment-1320394505
import processorUrl from './sineWaveSpeechProcessor.ts?worker&url'
import {
  getDefaultSynthesisParameters,
  SynthesisParameters,
} from './synthesisParameters.ts'
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

const synthesisParameters = ref<SynthesisParameters>(getDefaultSynthesisParameters())

const totalNumHops = computed(() => {
  const source = audioSourceNode.value
  if (source instanceof AudioBufferSourceNode) {
    // Pre-recorded source. we know how many hops there are.
    if (source.buffer === null) throw new Error('Buffer is null')
    const hopSize = BLOCK_SIZE * synthesisParameters.value.hopSizeMultiplier
    return Math.ceil(source.buffer.length / hopSize)
  } else if (source instanceof MediaStreamAudioSourceNode) {
    // Live source - a microphone.
    if (playbackState.value === PlaybackState.Recording) {
      return RECORDING_DURATION_BLOCKS / synthesisParameters.value.hopSizeMultiplier
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

/**
 * A more accurate check than playbackState because it's only true once the
 * user has given permission to use the microphone.
 */
const isActuallyRecording = computed(
  () => audioSourceNode.value instanceof MediaStreamAudioSourceNode
)

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

const updateSynthesisParameters = async (newParameters: SynthesisParameters) => {
  const { audioContext, sineWaveSpeechNode } = await getAudioSetup()
  for (const [parameterName, value] of Object.entries(newParameters)) {
    const param = sineWaveSpeechNode.parameters.get(parameterName)
    if (param === undefined) throw new Error(`Parameter ${parameterName} not found`)

    param.setValueAtTime(value, audioContext.currentTime)
  }
}
// deep: true is necessary because synthesisParameters is an object, we need to watch its properties
watch(synthesisParameters, updateSynthesisParameters, { deep: true })

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

  // The watch() call above only happen when the value is updated, so if the user didn't change
  // anything, the parameters might be out of sync with the audio processor.
  updateSynthesisParameters(synthesisParameters.value)

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
    bufferSource.connect(audioContext.destination)
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

const playAndAllowAudio = async () => {
  const { audioContext } = await getAudioSetup()
  playbackState.value = PlaybackState.PlayingRecorded
  await audioContext.resume()
}
</script>

<template>
  <div
    class="w-full min-h-screen items-center content-center justify-items-center grid grid-cols-1"
  >
    <div
      class="grid grid-cols-1 content-center justify-items-center font-body gap-4 max-w-sm p-2 h-full"
    >
      <h1 class="text-5xl italic font-header">Sine Wave Speech</h1>
      <p>
        Anything can be music if you listen closely enough.
        <span class="font-bold">Press Record or Play to start.</span>
      </p>
      <PlaybackControls
        v-model="playbackState"
        :playAndAllowAudio="playAndAllowAudio"
        :audioContextState="audioSetup?.audioContext.state"
      />

      <div
        class="h-2 bg-white overflow-hidden rounded-sm w-full"
        :style="{ '--recording-duration': `${RECORDING_DURATION_SEC}s` }"
      >
        <div
          class="progress-bar-animation bg-accent1 w-full h-full"
          v-if="isActuallyRecording"
        ></div>
      </div>

      <SynthesisParameterControls v-model="synthesisParameters" />

      <div class="max-w-3xl">
        <LiveVisualization
          :hops="hops"
          :totalNumHops="totalNumHops"
          :sampleRate="SAMPLE_RATE"
        />
      </div>
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
