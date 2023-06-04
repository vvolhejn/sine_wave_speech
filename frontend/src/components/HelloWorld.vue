<script setup lang="ts">
import { onMounted, ref } from 'vue'
import sentence from '../assets/sentence-original.wav'
import swsData from '../assets/sentence-sine-wave.json'

const audioContext = new window.AudioContext()

// These get set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)
const playButton = ref<HTMLButtonElement | null>(null)

const isPlaying = ref(false)

onMounted(() => {
  if (!audioElement.value) return

  const track = audioContext.createMediaElementSource(audioElement.value)
  track.connect(audioContext.destination)
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
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
