<script setup lang="ts">
import * as d3 from 'd3'
import { onMounted, ref, watch } from 'vue'

const props = defineProps({
  frequency: {
    type: Number,
    default: 1,
  },
  magnitude: {
    type: Number,
    default: 1,
  },
})

const path = ref<any>(null)

onMounted(() => {
  const svg = d3.select('#wave')
  path.value = svg.append('path')
  console.log(path.value)
  makePlot()
})

const makePlot = () => {
  const width = 800
  const height = 400
  const margin = { top: 20, right: 20, bottom: 30, left: 50 }

  const svg = d3.select('#wave').attr('width', width).attr('height', height)

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
    .attr('stroke', 'steelblue')
    .attr('stroke-width', 1.5)
    .attr(
      'd',
      d3
        .line()
        .x((d) => xScale(d))
        .y((d) => yScale(Math.sin(d * props.frequency) * Math.sqrt(props.magnitude)))
    )
}

watch(
  () => props.frequency,
  () => {
    if (!path.value) return
    makePlot()
  }
)
</script>
<template><svg id="wave"></svg></template>
