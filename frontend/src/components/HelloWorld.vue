<script setup lang="ts">
import { onMounted, ref } from 'vue'
import sentence from '../assets/sentence-original.wav'
import swsData from '../assets/sentence-sine-wave.json'
import SineWave from './SineWave.vue'
import { playSineWaveSpeech } from '../audio'
// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784

const audioContext = new window.AudioContext()

// These get set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)
const playButton = ref<HTMLButtonElement | null>(null)

const isPlaying = ref(false)
const visualizationFrequency = ref(1)
const visualizationMagnitude = ref(1)
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

const onClickSineWaveSpeech = () => {
  startTime.value = audioContext.currentTime
  playSineWaveSpeech(audioContext, swsData, audioContext.currentTime)
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
  const smoothedMagnitudes = smoothe(
    swsData.magnitudes.map((x) => x[0]),
    10
  )

  if (index < swsData.frequencies.length) {
    visualizationFrequency.value = (smoothedFrequencies[index] + 500) / 500
    visualizationMagnitude.value = smoothedMagnitudes[index]
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
  <button @click="onClickSineWaveSpeech">Play Sine Wave Speech</button>
  <SineWave :frequency="visualizationFrequency" :magnitude="visualizationMagnitude" />
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
