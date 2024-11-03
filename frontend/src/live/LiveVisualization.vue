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
  LineElement
)

const props = defineProps<{
  hops: Hop[]
}>()

const chartData = computed(() => {
  const hops = props.hops

  if (hops.length === 0) {
    return { labels: [], datasets: [{ data: [] }] }
  }

  const nWaves = hops[0].frequencies.length

  const datasets = [...Array(nWaves).keys()].map((i) => {
    const curFrequencies = hops.map((hop) => hop.frequencies[i])
    // const curMagnitudes = hops.map((hop) => hop.magnitudes[i])

    return { data: curFrequencies }
  })

  return {
    labels: [...Array(hops.length).keys()],
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
