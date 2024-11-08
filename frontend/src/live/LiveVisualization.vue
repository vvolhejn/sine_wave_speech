<script setup lang="ts">
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Colors,
} from 'chart.js'
import { computed, ref } from 'vue'
import { Line } from 'vue-chartjs'

import { Hop } from './types.ts'

ChartJS.register(
  Title,
  Tooltip,
  Legend,
  CategoryScale,
  LinearScale,
  PointElement,
  LineElement,
  Colors // Automatically colors the lines
)

const props = defineProps<{
  hops: Hop[]
  totalNumHops: number | null
  sampleRate: number
}>()

// If there is no data, the chart looks weird for an instant, leading to a flicker.
// This is a workaround to prevent that.
const DUMMY_HOPS = [
  {
    frequencies: new Float32Array([0, 0, 0, 0]),
    magnitudes: new Float32Array([0, 0, 0, 0]),
  },
]

const normalizedFrequencyToHz = (normalizedFrequency: number) =>
  (normalizedFrequency * props.sampleRate) / (2 * Math.PI)

const chartData = computed(() => {
  const hops = props.hops.length > 0 ? props.hops : DUMMY_HOPS

  const nWaves = hops[0].frequencies.length

  if (props.totalNumHops && hops.length > props.totalNumHops) {
    throw new Error('hops.length > totalNumHops')
  }

  const datasets = [...Array(nWaves).keys()].map((i) => {
    const curFrequencies = hops.map((hop) =>
      hop.frequencies[i] > 0 ? normalizedFrequencyToHz(hop.frequencies[i]) : null
    )
    // const curMagnitudes = hops.map((hop) => hop.magnitudes[i])

    return {
      data: curFrequencies,
      label: `${i + 1}`,
    }
  })

  return {
    labels: [...Array(props.totalNumHops || hops.length).keys()],
    datasets: datasets,
  }
})
const chartOptions = ref({
  responsive: true,
  animation: {
    duration: 0,
  },
  scales: {
    y: {
      min: 0,
      max: normalizedFrequencyToHz(Math.PI),
    },
  },
  plugins: {
    legend: {
      display: false,
    },
  },
})
</script>

<template>
  <Line id="my-chart-id" :options="chartOptions" :data="chartData" />
</template>
