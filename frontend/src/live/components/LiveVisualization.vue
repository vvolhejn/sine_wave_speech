<script setup lang="ts">
import * as d3 from 'd3'
import { onMounted, ref, watch } from 'vue'

import { magnitudeToDbfs } from '../audioUtils'
import type { Hop } from '@/live/types'
import { accentColors } from '@/tailwindColors'

const props = defineProps<{
  hops: Hop[]
  totalNumHops: number | null
  sampleRate: number
}>()

const svgRef = ref<SVGSVGElement | null>(null)
const margin = { top: 20, right: 20, bottom: 30, left: 40 }
const width = 800 - margin.left - margin.right
const height = 400 - margin.top - margin.bottom

const normalizedFrequencyToHz = (normalizedFrequency: number) =>
  (normalizedFrequency * props.sampleRate) / (2 * Math.PI)

const DUMMY_HOPS = [
  {
    frequencies: new Float32Array([0, 0, 0, 0]),
    magnitudes: new Float32Array([0, 0, 0, 0]),
  },
]

const updateVisualization = (clear: boolean = false) => {
  if (!svgRef.value) return

  const svg = d3.select(svgRef.value).select('g')

  const hops = props.hops.length > 0 ? props.hops : DUMMY_HOPS
  // nWaves can be different for different hops if the user modifies the number of waves
  const nWaves = Math.max(...hops.map((hop) => hop.frequencies.length))
  const numHops = props.totalNumHops || hops.length

  const xScale = d3.scaleLinear().domain([0, numHops]).range([0, width])

  const yScale = d3
    .scaleLinear()
    .domain([0, normalizedFrequencyToHz(Math.PI)])
    .range([height, 0])

  const magnitudeScale = d3.scaleLinear().domain([-60, 0]).range([0.1, 1])

  const linesToDraw = [...Array(nWaves).keys()].map((i) => {
    const frequencies = hops.map((hop) =>
      hop.frequencies[i] > 0 ? normalizedFrequencyToHz(hop.frequencies[i]) : null
    )
    // TODO: fix Rust code to ensure the magnitudes are always in [0, 1]
    const magnitudesDbfs = hops.map((hop) =>
      magnitudeToDbfs(Math.min(hop.magnitudes[i], 1))
    )

    return { frequencies, magnitudesDbfs }
  })

  const withOpacity = (color: d3.RGBColor, opacity: number) =>
    `rgba(${color.r}, ${color.g}, ${color.b}, ${opacity})`

  if (props.hops.length == 1) {
    clear = true // Also clear when we've started a new loop.
  }
  if (clear) {
    svg.selectAll('.line-any').remove()
    svg.selectAll('.circle-any').remove()
  }

  // Draw white dots at the bottom
  for (let i = 0; i < hops.length - 1; i++) {
    if (svg.select(`.circle-${i}`).size() > 0) {
      continue
    }
    svg
      .append('circle')
      .attr('cx', xScale(i))
      .attr('cy', yScale(0))
      .attr('r', 2)
      .attr('fill', 'white')
      .attr('class', `circle-${i} circle-any`)
  }

  // Update visualization for each line
  linesToDraw.forEach((dataset, datasetIndex) => {
    const baseColor = d3.color(accentColors[datasetIndex % accentColors.length])!.rgb()

    // Draw line segments
    for (let i = 0; i < dataset.frequencies.length - 1; i++) {
      // if the line already exists, do not redraw
      if (svg.select(`.line-${datasetIndex}-${i}`).size() > 0) {
        continue
      }

      let freq1 = dataset.frequencies[i]
      let freq2 = dataset.frequencies[i + 1]
      if (freq1 !== null && freq2 !== null) {
        svg
          .append('path')
          .attr('class', `line-${datasetIndex}-${i} line-any`)
          .attr(
            'd',
            d3.line()([
              [xScale(i), yScale(freq1)],
              [xScale(i + 1), yScale(freq2)],
            ])
          )
          .attr(
            'stroke',
            withOpacity(baseColor, magnitudeScale(dataset.magnitudesDbfs[i]))
          )
          .attr('stroke-width', 1 + 5 * magnitudeScale(dataset.magnitudesDbfs[i]))
          .attr('fill', 'none')
      }
    }
  })
}

onMounted(() => {
  if (!svgRef.value) return

  // Initialize SVG
  const svg = d3
    .select(svgRef.value)
    .attr('width', width + margin.left + margin.right)
    .attr('height', height + margin.top + margin.bottom)
    .attr(
      'viewBox',
      `0 0 ${width + margin.left + margin.right} ${height + margin.top + margin.bottom}`
    )

  svg.append('g').attr('transform', `translate(${margin.left},${margin.top})`)

  updateVisualization()
})

// Watch for changes in props
watch(
  () => props.hops,
  () => updateVisualization(false),
  { deep: true }
)
// When totalNumHops changes, we need to redraw the entire visualization
watch(
  () => props.totalNumHops,
  () => updateVisualization(true)
)
</script>

<template>
  <div class="w-full h-full">
    <svg ref="svgRef" class="w-full"></svg>
  </div>
</template>
