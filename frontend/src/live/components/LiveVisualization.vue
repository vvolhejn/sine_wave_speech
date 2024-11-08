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

const updateVisualization = () => {
  if (!svgRef.value) return

  const svg = d3.select(svgRef.value).select('g')

  const hops = props.hops.length > 0 ? props.hops : DUMMY_HOPS
  const nWaves = hops[0].frequencies.length
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

  // Update visualization for each line
  linesToDraw.forEach((dataset, datasetIndex) => {
    // Remove existing line segments
    svg.selectAll(`.line-${datasetIndex}`).remove()

    const baseColor = d3.color(accentColors[datasetIndex])!.rgb()

    // Draw line segments
    for (let i = 0; i < dataset.frequencies.length - 1; i++) {
      let freq1 = dataset.frequencies[i]
      let freq2 = dataset.frequencies[i + 1]
      if (freq1 !== null && freq2 !== null) {
        svg
          .append('path')
          .attr('class', `line-${datasetIndex}`)
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

  svg.append('g').attr('transform', `translate(${margin.left},${margin.top})`)

  updateVisualization()
})

// Watch for changes in props
watch(
  () => [props.hops, props.totalNumHops],
  () => updateVisualization(),
  { deep: true }
)
</script>

<template>
  <div class="w-full h-full">
    <svg ref="svgRef"></svg>
  </div>
</template>
