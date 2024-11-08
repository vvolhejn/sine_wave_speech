<script setup lang="ts">
import { ref } from 'vue'

import sentenceAudio from '../assets/sentence-original.wav'
import wasmUrl from '../wasm_realtime_sws/wasm_audio_bg.wasm?url'
import LiveVisualization from './LiveVisualization.vue'
import SineWaveSpeechNode from './sineWaveSpeechNode.ts'
// Importing with "?worker&url" and not "?url" is necessary:
// https://github.com/vitejs/vite/issues/6979#issuecomment-1320394505
import processorUrl from './sineWaveSpeechProcessor.ts?worker&url'
import { Hop } from './types.ts'

// Single source of truth for the recording duration (in seconds)
const RECORDING_DURATION_SEC = 3

// Must be a multiple of 128, the WebAudio block size
const HOP_SIZE = 256

const getAudioBuffer = async (audioContext: AudioContext, audioFile: string) => {
  const response = await fetch(audioFile)
  const arrayBuffer = await response.arrayBuffer()
  const audioBuffer = await audioContext.decodeAudioData(arrayBuffer)
  return audioBuffer
}

const getWebAudioMediaStream = async () => {
  if (!window.navigator.mediaDevices) {
    throw new Error('This browser does not support web audio or it is not enabled.')
  }

  try {
    const result = await window.navigator.mediaDevices.getUserMedia({
      audio: true,
      video: false,
    })

    return result
  } catch (e: any) {
    switch (e.name) {
      case 'NotAllowedError':
        throw new Error(
          'A recording device was found but has been disallowed for this application. Enable the device in the browser settings.'
        )

      case 'NotFoundError':
        throw new Error(
          'No recording device was found. Please attach a microphone and click Retry.'
        )

      default:
        throw e
    }
  }
}

const hops = ref<Hop[]>([])
const recordedAudioBuffer = ref<AudioBuffer | null>(null)
const isRecording = ref(false)
const totalNumHops = ref<number | null>(null)

const setupAudio = async () => {
  // The sample rate heavily affects the sine wave speech effect, 8000 is the tested one.
  const audioContext = new window.AudioContext({ sampleRate: 8000 })

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
      processorOptions: {
        hopSize: HOP_SIZE,
      },
    }
  )
  return { audioContext, sineWaveSpeechNode }
}

const startPlayingAudio = async (fromMicrophone: boolean) => {
  const { audioContext, sineWaveSpeechNode } = await setupAudio()
  sineWaveSpeechNode.connect(audioContext.destination)

  if (fromMicrophone) {
    const mediaStream = await getWebAudioMediaStream()
    const source = audioContext.createMediaStreamSource(mediaStream)
    source.connect(sineWaveSpeechNode)
  } else {
    const dryAudioBuffer =
      recordedAudioBuffer.value || (await getAudioBuffer(audioContext, sentenceAudio))
    let bufferSource = audioContext.createBufferSource()
    bufferSource.buffer = dryAudioBuffer
    bufferSource.connect(sineWaveSpeechNode)
    bufferSource.start()
    totalNumHops.value = Math.ceil(dryAudioBuffer.length / HOP_SIZE)
  }

  hops.value = []
  sineWaveSpeechNode.hops = hops
}

const startRecordingAudio = async () => {
  const { audioContext } = await setupAudio()
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
  <div class="grid grid-cols-1 content-center h-screen">
    <p>Work in progress, stay tuned.</p>
    <button
      @click="() => startPlayingAudio(true)"
      class="button text-center grid-auto text-3xl p-10"
    >
      Real-time
    </button>
    <button
      @click="() => startRecordingAudio()"
      class="button text-center grid-auto text-3xl p-10"
    >
      Record
    </button>
    <button
      @click="() => startPlayingAudio(false)"
      class="button text-center grid-auto text-3xl p-10"
    >
      From file
    </button>
    <div
      class="mt-2 h-2 bg-white overflow-hidden rounded-sm"
      :style="{ '--recording-duration': `${RECORDING_DURATION_SEC}s` }"
    >
      <div
        class="progress-bar-animation bg-accent1 w-full h-full"
        v-if="isRecording"
      ></div>
    </div>
    <div class="bg-white max-w-3xl">
      <LiveVisualization :hops="hops" :totalNumHops="totalNumHops" />
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
