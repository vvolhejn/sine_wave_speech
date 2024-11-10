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
import Slider from './components/Slider.vue'
import SineWaveSpeechNode from './sineWaveSpeechNode.ts'
// Importing with "?worker&url" and not "?url" is necessary:
// https://github.com/vitejs/vite/issues/6979#issuecomment-1320394505
import processorUrl from './sineWaveSpeechProcessor.ts?worker&url'
import { Hop } from './types.ts'

// Single source of truth for the recording duration (in seconds)
const RECORDING_DURATION_SEC = 3

// See BLOCK_SIZE in sineWaveSpeechProcessor.ts.
// We can't import that here because it's a Worker (I think?).
const BLOCK_SIZE = 128

// The sample rate significantly affects how sine wave speech effect sounds.
// 8000 is the tested one.
const SAMPLE_RATE = 8000

const TOTAL_NUM_HOPS_WHEN_LIVE = 48

const hops = ref<Hop[]>([])
const recordedAudioBuffer = ref<AudioBuffer | null>(null)
const isRecording = ref(false)
const audioSourceNode = ref<MediaStreamAudioSourceNode | AudioBufferSourceNode | null>(
  null
)

// Synthesis parameters
const nWaves = ref(4)
const frequencyQuantizationLevel = ref(0)
const hopSizeMultiplier = ref(2)
const gainDb = ref(0)

const totalNumHops = computed(() => {
  const source = audioSourceNode.value
  if (source instanceof AudioBufferSourceNode) {
    if (source.buffer === null) throw new Error('Buffer is null')
    const hopSize = BLOCK_SIZE * hopSizeMultiplier.value
    return Math.ceil(source.buffer.length / hopSize)
  } else if (source instanceof MediaStreamAudioSourceNode) {
    return TOTAL_NUM_HOPS_WHEN_LIVE
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

const setFrequencyQuantizationLevel = async (newFrequencyQuantizationLevel: number) => {
  updateParameter('frequencyQuantizationLevel', newFrequencyQuantizationLevel)
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

watch(frequencyQuantizationLevel, setFrequencyQuantizationLevel)
watch(hopSizeMultiplier, setHopSizeMultiplier)
watch(nWaves, setNWaves)
watch(gainDb, setGainDb)

// See also getFrequencyQuantizationType() in sineWaveSpeechProcessor.ts.
const frequencyQuantizationName = computed(() => {
  const level = frequencyQuantizationLevel.value
  switch (level) {
    case 0:
      return 'No quantization'
    case 1:
      return 'Chromatic'
    case 2:
      return 'Diatonic'
    case 3:
      return 'Pentatonic'
    default:
      throw new Error(`Invalid frequency quantization level ${level}`)
  }
})

const audioPlaying = ref(false)
watch(audioPlaying, async (newAudioPlaying: boolean) => {
  const { audioContext } = await getAudioSetup()
  if (newAudioPlaying) {
    await audioContext.resume()
  } else {
    await audioContext.suspend()
  }
})

const onPlayPause = async () => {
  if (audioSourceNode.value === null) {
    await startPlayingAudio(false)
  } else {
    audioPlaying.value = !audioPlaying.value
  }
}

const startPlayingAudio = async (fromMicrophone: boolean) => {
  const { audioContext, sineWaveSpeechNode } = await getAudioSetup()
  sineWaveSpeechNode.connect(audioContext.destination)

  // The watch() calls above only happen when the value is updated, so if the user didn't change
  // anything, the parameters might be out of sync with the audio processor.
  setFrequencyQuantizationLevel(frequencyQuantizationLevel.value)
  setHopSizeMultiplier(hopSizeMultiplier.value)
  setNWaves(nWaves.value)
  setGainDb(gainDb.value)

  if (fromMicrophone) {
    const mediaStream = await getWebAudioMediaStream()
    const source = audioContext.createMediaStreamSource(mediaStream)
    source.connect(sineWaveSpeechNode)
    audioSourceNode.value = source
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
    audioSourceNode.value = bufferSource
  }

  hops.value = []
  audioPlaying.value = true
}

const startRecordingAudio = async () => {
  const { audioContext } = await getAudioSetup()
  const mediaStream = await getWebAudioMediaStream()
  const mediaRecorder = new MediaRecorder(mediaStream)
  const audioChunks: BlobPart[] = []

  mediaRecorder.ondataavailable = (event) => {
    audioChunks.push(event.data)
  }

  mediaRecorder.onstop = async () => {
    const audioBlob = new Blob(audioChunks)
    const arrayBuffer = await audioBlob.arrayBuffer()
    recordedAudioBuffer.value = await audioContext.decodeAudioData(arrayBuffer)
    startPlayingAudio(false)
  }

  mediaRecorder.start()
  isRecording.value = true

  setTimeout(() => {
    isRecording.value = false
    mediaRecorder.stop()
  }, RECORDING_DURATION_SEC * 1000)
}
</script>

<template>
  <div class="grid grid-cols-1 content-center justify-items-center h-screen">
    <p>Work in progress, stay tuned.</p>
    <button
      @click="() => startPlayingAudio(true)"
      class="button grid-auto text-3xl p-10"
    >
      Real-time
    </button>
    <button @click="() => startRecordingAudio()" class="button grid-auto text-3xl p-10">
      Record
    </button>
    <button @click="onPlayPause" class="button grid-auto text-3xl p-10">
      {{ audioPlaying ? 'Pause' : 'Play' }}
    </button>
    <div>
      <Slider
        v-model="nWaves"
        :label="`Number of waves: ${nWaves}`"
        :min="1"
        :max="16"
        id="n-waves-slider"
      />
      <Slider
        v-model="frequencyQuantizationLevel"
        :label="frequencyQuantizationName"
        :min="0"
        :max="3"
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
        :label="`Gain (volume): ${gainDb} dB`"
        :min="-12"
        :max="12"
        id="gain-db-slider"
      />
    </div>
    <div
      class="mt-2 h-2 bg-white overflow-hidden rounded-sm w-full"
      :style="{ '--recording-duration': `${RECORDING_DURATION_SEC}s` }"
    >
      <div
        class="progress-bar-animation bg-accent1 w-full h-full"
        v-if="isRecording"
      ></div>
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
