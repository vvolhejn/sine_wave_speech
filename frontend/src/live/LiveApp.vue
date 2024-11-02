<script setup lang="ts">
import sentenceAudio from '../assets/sentence-original.wav'
import wasmUrl from '../wasm_realtime_sws/wasm_audio_bg.wasm?url'
import SineWaveSpeechNode from './sineWaveSpeechNode.js'
// Importing with "?worker&url" and not "?url" is necessary:
// https://github.com/vitejs/vite/issues/6979#issuecomment-1320394505
import processorUrl from './sineWaveSpeechProcessor.ts?worker&url'

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
    wasmBytes
  )
  return { audioContext, sineWaveSpeechNode }
}

const startPlayingAudio = async (fromMicrophone: boolean) => {
  const { audioContext, sineWaveSpeechNode } = await setupAudio()

  let source: AudioNode

  if (fromMicrophone) {
    const mediaStream = await getWebAudioMediaStream()
    source = audioContext.createMediaStreamSource(mediaStream)
  } else {
    const dryAudioBuffer = await getAudioBuffer(audioContext, sentenceAudio)
    let bufferSource = audioContext.createBufferSource()
    bufferSource.buffer = dryAudioBuffer
    source = bufferSource
    bufferSource.start()
  }

  source.connect(sineWaveSpeechNode).connect(audioContext.destination)
}
</script>

<template>
  <div class="grid grid-cols-1 content-center h-screen">
    <p>Work in progress, stay tuned.</p>
    <button
      @click="() => startPlayingAudio(true)"
      class="button text-center grid-auto text-5xl p-20"
    >
      From microphone
    </button>
    <button
      @click="() => startPlayingAudio(false)"
      class="button text-center grid-auto text-5xl p-20"
    >
      From file
    </button>
  </div>
</template>
