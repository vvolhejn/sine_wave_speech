<script setup lang="ts">
import * as d3 from 'd3'
import { onMounted, ref, watch } from 'vue'
import { usePlaybackStore } from '../stores/playbackStore'

const props = defineProps<{ waveIndex: number; path: any }>()

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

const smoothe = (arr: number[], windowLength: number) => {
  const result = new Array<number>(arr.length)
  for (let i = 0; i < arr.length; i++) {
    const start = Math.max(0, i - windowLength)
    const end = Math.min(arr.length, i + windowLength)
    result[i] = arr.slice(start, end).reduce((a, b) => a + b, 0) / (end - start)
  }
  return result
}

const getFrequencyAndMagnitude = () => {
  const index = playbackStore.swsIndex
  if (index == null) {
    return [null, null]
  }

  const swsData = playbackStore.swsData
  if (swsData == null) {
    return [null, null]
  }

  const smoothedFrequencies = smoothe(
    swsData.frequencies.map((x) => x[props.waveIndex]),
    10
  )
  const smoothedMagnitudes = smoothe(
    swsData.magnitudes.map((x) => x[props.waveIndex]),
    10
  )

  const frequency = smoothedFrequencies[index]
  const magnitude = smoothedMagnitudes[index]

  return [frequency, magnitude]
}

const makePlot = () => {
  const width = 800
  const height = 400
  const margin = { top: 20, right: 20, bottom: 30, left: 50 }

  const [frequency, magnitude] = getFrequencyAndMagnitude()
  if (frequency == null || magnitude == null) {
    return
  }
  const scaledFrequency = (frequency + 500) / 500
  const offset = playbackStore.animationTime * 10

  const svg = d3.select('#visualization').attr('width', width).attr('height', height)

  const xScale = d3
    .scaleLinear()
    .domain([-Math.PI + offset, Math.PI + offset])
    .range([margin.left, width - margin.right])

  const yScale = d3
    .scaleLinear()
    .domain([-1, 1])
    .range([height - margin.bottom, margin.top])

  path.value
    .datum(d3.range(-Math.PI + offset, Math.PI + offset, 0.01))
    .attr('fill', 'none')
    .attr('stroke', 'steelblue')
    .attr('stroke-width', 1.5)
    .attr(
      'd',
      d3
        .line()
        .x((d) => xScale(d))
        .y((d) =>
          yScale(
            Math.sin((d - offset) * scaledFrequency + offset) * Math.sqrt(magnitude) +
              props.waveIndex * 0.3
          )
        )
    )
}
</script>
<template><svg ref=""></svg></template>
