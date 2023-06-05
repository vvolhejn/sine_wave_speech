<script setup lang="ts">
import * as d3 from 'd3'
import { onMounted, ref, watch } from 'vue'
import { usePlaybackStore } from '../stores/playbackStore'
import { WaveConfig } from '../types'

const props = defineProps<{ waveConfig: WaveConfig }>()

const path = ref<any>(null)

onMounted(() => {
  const svg = d3.select('#visualization')
  path.value = svg.append('path')
  makePlot()
})

const playbackStore = usePlaybackStore()
watch(
  () => playbackStore.animationTime,
  () => {
    makePlot()
  }
)

const getFrequencyAndMagnitude = () => {
  const index = playbackStore.swsIndex
  if (index == null) {
    return [null, null]
  }

  const swsData = playbackStore.swsData
  if (swsData == null) {
    return [null, null]
  }

  const smoothedFrequencies = playbackStore.smoothedFrequencies
  const smoothedMagnitudes = playbackStore.smoothedMagnitudes
  if (smoothedFrequencies == null || smoothedMagnitudes == null) {
    console.log('bad')
    return [null, null]
  }
  const frequency = smoothedFrequencies[props.waveConfig.waveIndex][index]
  const magnitude = smoothedMagnitudes[props.waveConfig.waveIndex][index]

  console.log(frequency, magnitude)

  return [frequency, magnitude]
}

const makePlot = () => {
  const width = document.body.clientWidth
  const height = document.body.clientHeight

  const margin = { top: 0, right: -5, bottom: 0, left: -5 }

  const [frequency, magnitude] = getFrequencyAndMagnitude()
  if (frequency == null || magnitude == null) {
    return
  }
  const scaledFrequency = (frequency + 500) / 1000
  const offset = playbackStore.animationTime * props.waveConfig.xSpeed * 2

  const xScale = d3
    .scaleLinear()
    .domain([-Math.PI, Math.PI])
    .range([margin.left, width - margin.right])

  const yScale = d3
    .scaleLinear()
    .domain([-1, 1])
    .range([height - margin.bottom, margin.top])

  path.value
    .datum(d3.range(-Math.PI, Math.PI, 0.01))
    .attr('fill', 'none')
    .attr('stroke', props.waveConfig.color || 'white')
    .attr('stroke-width', 4)
    .attr(
      'd',
      d3
        .line()
        .x((d) => xScale(d))
        .y((d) =>
          yScale(
            Math.sin(d * scaledFrequency + offset) * Math.sqrt(magnitude) +
              props.waveConfig.yOffset
          )
        )
    )
}
</script>
<template><svg ref=""></svg></template>
