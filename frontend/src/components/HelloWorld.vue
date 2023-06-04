<script setup lang="ts">
import { compileToFunction, onMounted, ref, watch } from 'vue'
import sentence from '../assets/sentence-original.wav'
import swsData from '../assets/sentence-sine-wave.json'
import SineWaveVisualization from './SineWaveVisualization.vue'
// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784

const audioContext = new window.AudioContext()

// These get set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)
const playButton = ref<HTMLButtonElement | null>(null)

const isPlaying = ref(false)
const coef = ref(1)
const startTime = ref(0)

onMounted(() => {
  if (!audioElement.value) return

  const track = audioContext.createMediaElementSource(audioElement.value)
  track.connect(audioContext.destination)
})

const onAudioEnded = () => {
  isPlaying.value = false
}

const onClick = () => {
  // Check if context is in suspended state (autoplay policy)
  if (audioContext.state === 'suspended') {
    audioContext.resume()
  }

  if (!audioElement.value) return

  // Play or pause track depending on state
  if (!isPlaying.value) {
    audioElement.value.play()
    isPlaying.value = true
  } else {
    audioElement.value.pause()
    isPlaying.value = false
  }
}

const playSineWaveSpeech = (time: number) => {
  startTime.value = audioContext.currentTime
  console.log(audioContext.currentTime)
  const oscillators = new Array<OscillatorNode>()
  const gains = new Array<GainNode>()
  const nWaves = swsData.frequencies[0].length
  const nTimesteps = swsData.frequencies.length

  for (let i = 0; i < nWaves; i++) {
    const osc = new OscillatorNode(audioContext, {
      frequency: swsData.frequencies[0][i],
      type: 'sine',
    })
    // swsData.magnitudes[0][i]
    const gain = new GainNode(audioContext, { gain: 0 })
    osc.connect(gain).connect(audioContext.destination)
    oscillators.push(osc)
    gains.push(gain)
  }

  // Check if context is in suspended state (autoplay policy)
  if (audioContext.state === 'suspended') {
    audioContext.resume()
  }
  const secondsPerTimestep = swsData.hopSize / swsData.sr

  for (let t = 0; t < nTimesteps; t++) {
    for (let i = 0; i < nWaves; i++) {
      const osc = oscillators[i]
      const gain = gains[i]
      osc.frequency.linearRampToValueAtTime(
        swsData.frequencies[t][i],
        time + t * secondsPerTimestep
      )
      gain.gain.linearRampToValueAtTime(
        swsData.magnitudes[t][i],
        time + t * secondsPerTimestep
      )
    }
  }

  oscillators.forEach((oscillator) => {
    oscillator.start(time)
    oscillator.stop(time + nTimesteps * secondsPerTimestep)
  })
}

const smoothe = (arr: number[], windowLength: number) => {
  const result = new Array<number>(arr.length)
  for (let i = 0; i < arr.length; i++) {
    const start = Math.max(0, i - windowLength)
    const end = Math.min(arr.length, i + windowLength)
    result[i] = arr.slice(start, end).reduce((a, b) => a + b, 0) / (end - start)
  }
  return result
}

const step = () => {
  if (startTime.value == null) {
    window.requestAnimationFrame(step)
    return
  }

  const secondsPerTimestep = swsData.hopSize / swsData.sr
  const index = Math.floor(
    (audioContext.currentTime - startTime.value) / secondsPerTimestep
  )

  const smoothedFrequencies = smoothe(
    swsData.frequencies.map((x) => x[0]),
    10
  )

  if (index < swsData.frequencies.length) {
    coef.value = (smoothedFrequencies[index] + 10000) / 5000
  }

  window.requestAnimationFrame(step)
}
window.requestAnimationFrame(step)
</script>

<template>
  <h1>Sine Wave Speech</h1>

  <audio :src="sentence" ref="audioElement" @ended="onAudioEnded"></audio>
  <button
    data-playing="false"
    role="switch"
    aria-checked="false"
    @click="onClick"
    ref="playButton"
  >
    <span>Play original</span>
  </button>
  <button @click="playSineWaveSpeech(audioContext.currentTime)">
    Play Sine Wave Speech
  </button>
  <SineWaveVisualization :coef="coef" />
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
