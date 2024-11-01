<script setup lang="ts">
import sentenceAudio from '../assets/sentence-original.wav'
import init, { SineWaveSpeechConverter } from '../wasm_realtime_sws/wasm_audio.js'

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

const handleClick = async () => {
  if (!converter) {
    console.log('Converter not initialized')
    return
  }

  const audioContext = new AudioContext({ sampleRate: 8000 })

  const dryAudioBuffer = await getAudioBuffer(audioContext, sentenceAudio)
  const converted = converter.convert(dryAudioBuffer.getChannelData(0))

  var wetAudioBuffer = audioContext.createBuffer(
    1,
    converted.length,
    audioContext.sampleRate
  )
  wetAudioBuffer.getChannelData(0).set(converted)

  var source = audioContext.createBufferSource()
  source.buffer = wetAudioBuffer

  // connect the AudioBufferSourceNode to the destination so we can hear the sound
  source.connect(audioContext.destination)
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
