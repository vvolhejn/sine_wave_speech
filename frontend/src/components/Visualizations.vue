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
    waveIndex: 3,
    yOffset: 0.6,
    color: tailwindConfig.theme.colors.accent4,
    xSpeed: 1.6,
  },
  {
    waveIndex: 2,
    yOffset: 0.2,
    color: tailwindConfig.theme.colors.accent3,
    xSpeed: 1.4,
  },
  {
    waveIndex: 1,
    yOffset: -0.2,
    color: tailwindConfig.theme.colors.accent2,
    xSpeed: 1.2,
  },
  {
    waveIndex: 0,
    yOffset: -0.6,
    color: tailwindConfig.theme.colors.accent1,
    xSpeed: 1,
  },
]
</script>
<template>
  <div class="fixed -z-10 top-0 left-0">
    <svg id="visualization" class="w-screen h-screen"></svg>
    <SineWave v-for="waveConfig in wavesConfig" :waveConfig="waveConfig" />
  </div>
</template>
