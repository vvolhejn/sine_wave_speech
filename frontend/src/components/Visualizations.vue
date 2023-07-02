<script setup lang="ts">
import { usePlaybackStore } from '../stores/playbackStore'
import SineWave from './SineWave.vue'
import { WaveConfig } from '../types'
import resolveConfig from 'tailwindcss/resolveConfig'
import partialTailwindConfig from '../../tailwind.config.js'

const tailwindConfig = resolveConfig(partialTailwindConfig)

const playbackStore = usePlaybackStore()

const step = () => {
  window.requestAnimationFrame(step)
  playbackStore.updateAnimationTime()

  const analyser = playbackStore.audioNodes?.analyser
  if (!analyser) return
}
window.requestAnimationFrame(step)

const X_SPEED_COEF = 1.2

const accentColors = [
  tailwindConfig.theme.colors.accent4,
  tailwindConfig.theme.colors.accent3,
  tailwindConfig.theme.colors.accent2,
  tailwindConfig.theme.colors.accent1,
]

const wavesConfig: WaveConfig[] = [0, 1, 2, 3].map((i) => {
  return {
    waveIndex: 3 - i,
    yOffset: 0.6 - i * 0.4,
    color: accentColors[i],
    xSpeed: X_SPEED_COEF ** (3 - i),
    frequencyWhenPaused: 4000 - 1000 * i,
  }
})
</script>
<template>
  <div class="fixed -z-10 top-0 left-0 w-screen h-screen" id="visualization-div">
    <svg id="visualization" class="w-full h-full"></svg>
    <SineWave v-for="waveConfig in wavesConfig" :waveConfig="waveConfig" />
  </div>
</template>
