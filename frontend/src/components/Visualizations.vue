<script setup lang="ts">
import { usePlaybackStore } from '../stores/playbackStore'
import SineWave from './SineWave.vue'
import * as d3 from 'd3'
import { WaveConfig } from '../types'
import resolveConfig from 'tailwindcss/resolveConfig'
import partialTailwindConfig from '../../tailwind.config.js'

const tailwindConfig = resolveConfig(partialTailwindConfig)

const playbackStore = usePlaybackStore()

const step = () => {
  window.requestAnimationFrame(step)
  playbackStore.updateAnimationTime()
}
window.requestAnimationFrame(step)

const svg = d3.select('#visualization')

const wavesConfig: WaveConfig[] = [
  {
    waveIndex: 0,
    yOffset: 1.2,
    color: tailwindConfig.theme.colors.accent4,
  },
  {
    waveIndex: 1,
    yOffset: 0.8,
    color: tailwindConfig.theme.colors.accent3,
  },
  {
    waveIndex: 2,
    yOffset: 0.4,
    color: tailwindConfig.theme.colors.accent2,
  },
  {
    waveIndex: 3,
    yOffset: 0.0,
    color: tailwindConfig.theme.colors.accent1,
  },
]
</script>
<template>
  <div class="fixed -z-10">
    <svg id="visualization" class="w-screen h-screen"></svg>
    <SineWave v-for="waveConfig in wavesConfig" :waveConfig="waveConfig" />
  </div>
</template>
