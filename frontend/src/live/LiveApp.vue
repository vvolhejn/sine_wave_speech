<script setup lang="ts">
import init, { WasmPitchDetector } from '../wasm_realtime_sws/wasm_audio.js'

const sampleRate = 8000
const frequency = 200
const fftSize = 2048
const length = fftSize
const sineWave = new Float32Array(length)

for (let i = 0; i < length; i++) {
  sineWave[i] = Math.sin((2 * Math.PI * frequency * i) / sampleRate) * 0.5
}

init({ module: '../wasm_realtime_sws/wasm_audio_bg.wasm' }).then(() => {
  console.log('Wasm initialized')
  let pitchDetector = WasmPitchDetector.new(sampleRate, fftSize)
  console.log(pitchDetector.detect_pitch(sineWave))
})
</script>

<template><p>Work in progress, stay tuned.</p></template>
