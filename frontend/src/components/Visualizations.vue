<script setup lang="ts">
import { ref } from 'vue'
import { usePlaybackStore } from '../stores/playbackStore'
import SineWave from './SineWave.vue'
import * as d3 from 'd3'

const playbackStore = usePlaybackStore()

const visualizationFrequency = ref(1)
const visualizationMagnitude = ref(1)

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
  window.requestAnimationFrame(step)
  playbackStore.updateAnimationTime()

  if (playbackStore.startTime == null) {
    return
  }

  const index = playbackStore.swsIndex
  if (index == null) {
    return
  }

  const swsData = playbackStore.swsData
  if (swsData == null) {
    return
  }

  const smoothedFrequencies = smoothe(
    swsData.frequencies.map((x) => x[0]),
    10
  )
  const smoothedMagnitudes = smoothe(
    swsData.magnitudes.map((x) => x[0]),
    10
  )

  visualizationFrequency.value = (smoothedFrequencies[index] + 500) / 500
  visualizationMagnitude.value = smoothedMagnitudes[index]
}
window.requestAnimationFrame(step)

const svg = d3.select('#visualization')

const wavesConfig = [
  {
    waveIndex: 0,
    yOffset: 1.2,
  },
  {
    waveIndex: 1,
    yOffset: 0.8,
  },
  {
    waveIndex: 2,
    yOffset: 0.4,
  },
  {
    waveIndex: 3,
    yOffset: 0.0,
  },
]
</script>
<template>
  <svg id="visualization"></svg>
  <SineWave
    v-for="waveConfig in wavesConfig"
    :waveIndex="waveConfig.waveIndex"
    :yOffset="waveConfig.yOffset"
  />
</template>
