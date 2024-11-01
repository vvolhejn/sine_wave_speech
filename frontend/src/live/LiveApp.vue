<script setup lang="ts">
import sentenceAudio from '../assets/sentence-original.wav'
import init, { SineWaveSpeechConverter } from '../wasm_realtime_sws/wasm_audio.js'
import wasmUrl from '../wasm_realtime_sws/wasm_audio_bg.wasm?url'
import SineWaveSpeechNode from './sineWaveSpeechNode.js'
import processorUrl from './sineWaveSpeechProcessor.ts?url'

const nWaves = 4
const hopSize = 256

let converter: SineWaveSpeechConverter | null = null

init({ module: '../wasm_realtime_sws/wasm_audio_bg.wasm' }).then(() => {
  converter = SineWaveSpeechConverter.new(nWaves, hopSize)
})

const getAudioBuffer = async (audioContext: AudioContext, audioFile: string) => {
  const response = await fetch(audioFile)
  const arrayBuffer = await response.arrayBuffer()
  const audioBuffer = await audioContext.decodeAudioData(arrayBuffer)
  return audioBuffer
}

async function setupAudio() {
  const audioContext = new window.AudioContext()

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

const handleClick = async () => {
  if (!converter) {
    console.log('Converter not initialized')
    return
  }

  const { audioContext, sineWaveSpeechNode } = await setupAudio()

  const dryAudioBuffer = await getAudioBuffer(audioContext, sentenceAudio)

  var source = audioContext.createBufferSource()
  source.buffer = dryAudioBuffer

  source.connect(sineWaveSpeechNode).connect(audioContext.destination)
  source.start()
}
</script>

<template>
  <div class="grid grid-cols-1 content-center h-screen">
    <p>Work in progress, stay tuned.</p>
    <button @click="handleClick" class="button text-center grid-auto text-5xl p-20">
      Click me!
    </button>
  </div>
</template>
