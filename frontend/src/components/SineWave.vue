<script setup lang="ts">
import * as d3 from 'd3'
import { onMounted, ref, watch } from 'vue'
import { usePlaybackStore } from '../stores/playbackStore'
import { WaveConfig } from '../types'
import { Smoother } from '../smoother'

const N_WAVES = 4
const DEFAULT_MAGNITUDE = 0.05

const props = defineProps<{ waveConfig: WaveConfig }>()

const path = ref<any>(null)

const frequencySmoother = new Smoother(0.2, props.waveConfig.frequencyWhenPaused)
const magnitudeSmoother = new Smoother(
  0.2,
  props.waveConfig.magnitudeWhenPaused || DEFAULT_MAGNITUDE
)

onMounted(() => {
  const svg = d3.select('#visualization')
  path.value = svg.append('path')
  makePlot(playbackStore.animationTime)
})

const playbackStore = usePlaybackStore()
watch(
  () => playbackStore.animationTime,
  (animationTime) => {
    makePlot(animationTime)
  }
)

const getFrequencyAndMagnitude = () => {
  const index = playbackStore.swsIndex
  const swsData = playbackStore.swsData
  if (index == null || swsData == null) {
    return [
      props.waveConfig.frequencyWhenPaused,
      props.waveConfig.magnitudeWhenPaused || DEFAULT_MAGNITUDE,
    ]
  }

  const frequency = swsData.frequencies[index][props.waveConfig.waveIndex]
  const magnitude = swsData.magnitudes[index][props.waveConfig.waveIndex]

  return [frequency, magnitude]
}

const makePlot = (animationTime: number) => {
  const visualizationDiv = document.getElementById('visualization-div')
  if (!visualizationDiv) {
    throw new Error("Couldn't find #visualization-div")
  }
  const width = visualizationDiv.clientWidth
  const height = visualizationDiv.clientHeight

  const margin = { top: 0, right: -5, bottom: 0, left: -5 }

  let [frequency, magnitude] = getFrequencyAndMagnitude()

  // Filter away outliers
  if (frequency > 100) {
    frequency = frequencySmoother.update(animationTime, frequency)
  } else {
    frequency = frequencySmoother.value
  }

  magnitude = magnitudeSmoother.update(animationTime, magnitude)

  const scaledFrequency = (frequency + 500) / 1000
  const offset = playbackStore.animationTime * props.waveConfig.xSpeed * 2

  const analyser = playbackStore.audioNodes?.analyser
  if (!analyser) return
  const bufferLength = analyser.frequencyBinCount
  const dataArray = new Uint8Array(bufferLength)
  playbackStore.audioNodes?.analyser.getByteTimeDomainData(dataArray)
  // console.log(dataArray)

  const xScale = d3
    .scaleLinear()
    .domain([-Math.PI, Math.PI])
    .range([margin.left, width - margin.right])

  const yScale = d3
    .scaleLinear()
    .domain([-1, 1])
    .range([height - margin.bottom, margin.top])

  const getY = (d: number) => {
    const scrollFraction = playbackStore.scrollFraction

    let y = Math.sin(d * scaledFrequency + offset) * Math.sqrt(magnitude) * 0.5

    // Offset the waves vertically, getting closer as you scroll down
    y += props.waveConfig.yOffset * (1 - scrollFraction)

    // Make each wave use a different part of the buffer so that they
    // don't all move in sync
    let xFraction = (d + Math.PI) / (2 * Math.PI)
    xFraction = xFraction / N_WAVES + props.waveConfig.waveIndex / N_WAVES
    const dataArrayIndex = Math.floor(xFraction * bufferLength)

    y += ((dataArray[dataArrayIndex] - 128) / 255) * scrollFraction

    return y
  }

  path.value
    .datum(d3.range(-Math.PI, Math.PI, 0.01))
    .attr('fill', 'none')
    .attr('stroke', props.waveConfig.color || 'white')
    .attr('stroke-width', 6)
    .attr(
      'd',
      d3
        .line()
        .x((d) => xScale(d))
        .y((d) => yScale(getY(d)))
    )
}

watch(() => [document.body.clientWidth, document.body.clientHeight], makePlot)
</script>
<template><svg ref=""></svg></template>
