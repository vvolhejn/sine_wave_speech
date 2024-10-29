<script setup lang="ts">
import init, {
  SineWaveSpeechConverter,
  SineWaveStep,
} from '../wasm_realtime_sws/wasm_audio.js'

const sampleRate = 8000
const frequency = 200
const fftSize = 2048
const nWaves = 4
const hopSize = 256
const length = fftSize
const sineWave = new Float32Array(length)

for (let i = 0; i < length; i++) {
  sineWave[i] = Math.sin((2 * Math.PI * frequency * i) / sampleRate) * 0.5
}

const unflattenSineWaveSpeechResult = (
  result: SineWaveStep[],
  nWaves: number
): SineWaveStep[][] => {
  const unflattened: SineWaveStep[][] = []
  for (let i = 0; i < result.length; i += nWaves) {
    unflattened.push(result.slice(i, i + nWaves))
  }
  return unflattened
}

init({ module: '../wasm_realtime_sws/wasm_audio_bg.wasm' }).then(() => {
  console.log('Wasm initialized')
  let converter = SineWaveSpeechConverter.new(nWaves, hopSize)
  const converted = unflattenSineWaveSpeechResult(
    converter.convert(sineWave),
    converter.n_waves
  )
  console.log(converted)
})
</script>

<template><p>Work in progress, stay tuned.</p></template>
