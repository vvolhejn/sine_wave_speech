<script setup lang="ts">
import { compileToFunction, onMounted, ref, watch } from 'vue'
import sentence from '../assets/sentence-original.wav'
import swsData from '../assets/sentence-sine-wave.json'
// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784

const audioContext = new window.AudioContext()

// These get set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)
const playButton = ref<HTMLButtonElement | null>(null)

const isPlaying = ref(false)
const path = ref<any>(null)
const coef = ref<number>(1)

onMounted(() => {
  if (!audioElement.value) return

  const track = audioContext.createMediaElementSource(audioElement.value)
  track.connect(audioContext.destination)

  const svg = d3.select('#wave')
  path.value = svg.append('path')
  console.log(path.value)
  makePlot()
})

const onAudioEnded = () => {
  isPlaying.value = false
}

const onClick = () => {
  // Check if context is in suspended state (autoplay policy)
  if (audioContext.state === 'suspended') {
    audioContext.resume()
  }

  if (!audioElement.value) return

  // Play or pause track depending on state
  if (!isPlaying.value) {
    audioElement.value.play()
    isPlaying.value = true
  } else {
    audioElement.value.pause()
    isPlaying.value = false
  }
}

const playSineWaveSpeech = (time: number) => {
  coef.value = 2

  const oscillators = new Array<OscillatorNode>()
  const gains = new Array<GainNode>()
  const nWaves = swsData.frequencies[0].length
  const nTimesteps = swsData.frequencies.length

  for (let i = 0; i < nWaves; i++) {
    const osc = new OscillatorNode(audioContext, {
      frequency: swsData.frequencies[0][i],
      type: 'sine',
    })
    // swsData.magnitudes[0][i]
    const gain = new GainNode(audioContext, { gain: 0 })
    osc.connect(gain).connect(audioContext.destination)
    oscillators.push(osc)
    gains.push(gain)
  }

  // Check if context is in suspended state (autoplay policy)
  if (audioContext.state === 'suspended') {
    audioContext.resume()
  }
  const secondsPerTimestep = swsData.hopSize / swsData.sr

  for (let t = 0; t < nTimesteps; t++) {
    for (let i = 0; i < nWaves; i++) {
      const osc = oscillators[i]
      const gain = gains[i]
      osc.frequency.linearRampToValueAtTime(
        swsData.frequencies[t][i],
        time + t * secondsPerTimestep
      )
      gain.gain.linearRampToValueAtTime(
        swsData.magnitudes[t][i],
        time + t * secondsPerTimestep
      )
    }
  }

  oscillators.forEach((oscillator) => {
    oscillator.start(time)
    oscillator.stop(time + nTimesteps * secondsPerTimestep)
  })
}

import * as d3 from 'd3'

const makePlot = () => {
  const width = 800
  const height = 400
  const margin = { top: 20, right: 20, bottom: 30, left: 50 }

  const svg = d3.select('#wave').attr('width', width).attr('height', height)

  const xScale = d3
    .scaleLinear()
    .domain([0, 2 * Math.PI])
    .range([margin.left, width - margin.right])

  const yScale = d3
    .scaleLinear()
    .domain([-1, 1])
    .range([height - margin.bottom, margin.top])

  path.value
    .datum(d3.range(0, 2 * Math.PI, 0.01))
    .attr('fill', 'none')
    .attr('stroke', 'steelblue')
    .attr('stroke-width', 1.5)
    .attr(
      'd',
      d3
        .line()
        .x((d) => xScale(d))
        .y((d) => yScale(Math.sin(d * coef.value)))
    )
}

watch(coef, () => {
  if (!path.value) return
  makePlot()
})
</script>

<template>
  <h1>Sine Wave Speech</h1>

  <audio :src="sentence" ref="audioElement" @ended="onAudioEnded"></audio>
  <button
    data-playing="false"
    role="switch"
    aria-checked="false"
    @click="onClick"
    ref="playButton"
  >
    <span>Play original</span>
  </button>
  <button @click="playSineWaveSpeech(audioContext.currentTime)">
    Play Sine Wave Speech
  </button>
  <svg id="wave"></svg>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
