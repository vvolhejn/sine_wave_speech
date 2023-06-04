<script setup lang="ts">
import { onMounted, ref } from 'vue'
import sentence from '../assets/sentence-original.wav'
import swsData from '../assets/sentence-sine-wave.json'
import { usePlaybackStore } from '../stores/playbackStore'
import Visualizations from './Visualizations.vue'
// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784
// https://coolors.co/1e152a-4e6766-5ab1bb-a5c882-f7dd72

// const audioContext = new window.AudioContext()
const playbackStore = usePlaybackStore()
playbackStore.setSwsData(swsData)

// These get set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)
const playButton = ref<HTMLButtonElement | null>(null)

onMounted(() => {
  if (!audioElement.value) return

  const track = playbackStore.audioContext.createMediaElementSource(audioElement.value)
  track.connect(playbackStore.audioContext.destination)
})

const onAudioEnded = () => {
  playbackStore.setIsPlaying(false)
}

const onClick = () => {
  // Check if context is in suspended state (autoplay policy)
  if (playbackStore.audioContext.state === 'suspended') {
    playbackStore.audioContext.resume()
  }

  if (!audioElement.value) return

  // Play or pause track depending on state
  if (!playbackStore.isPlaying) {
    audioElement.value.play()
    playbackStore.setIsPlaying(true)
  } else {
    audioElement.value.pause()
    playbackStore.setIsPlaying(false)
  }
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
  <button @click="playbackStore.playSineWaveSpeech">Play Sine Wave Speech</button>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
