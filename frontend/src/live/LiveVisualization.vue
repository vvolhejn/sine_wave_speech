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
}>()

const chartData = computed(() => {
  const hops = props.hops

  if (hops.length === 0) {
    return { labels: [], datasets: [{ data: [] }] }
  }

  const nWaves = hops[0].frequencies.length

  if (props.totalNumHops && hops.length > props.totalNumHops) {
    throw new Error('hops.length > totalNumHops')
  }

  const datasets = [...Array(nWaves).keys()].map((i) => {
    const curFrequencies = hops.map((hop) =>
      hop.frequencies[i] > 0 ? hop.frequencies[i] : null
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
})
</script>

<template>
  <Line id="my-chart-id" :options="chartOptions" :data="chartData" />
</template>
