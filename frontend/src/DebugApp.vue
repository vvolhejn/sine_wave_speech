<!-- A simple WebAudio thingy that ACTUALLY WORKS on iOS -->
<script setup lang="ts">
// We don't care about types here
// @ts-nocheck
import { ref, onBeforeUnmount } from 'vue'

import { originalAudio } from './dataFiles'

// const audioContext = ref<AudioContext | null>(null)
const audioBuffer = ref(null)
const sourceNode = ref(null)
const isPlaying = ref(false)
const isReady = ref(false)
const startTime = ref(0)
const offset = ref(0)

const initializeAudioContext = async () => {
  try {
    // Create audio context with auto-play policy for iOS Safari
    const audioContext = new window.AudioContext()

    // Fetch and decode audio file
    const response = await fetch(originalAudio)
    const arrayBuffer = await response.arrayBuffer()
    audioBuffer.value = await audioContext.decodeAudioData(arrayBuffer)

    isReady.value = true

    return audioContext
  } catch (error) {
    console.error('Error initializing audio:', error)
  }
}

const createAndStartSource = (audioContext: AudioContext) => {
  // Create new source node
  sourceNode.value = audioContext.createBufferSource()
  sourceNode.value.buffer = audioBuffer.value
  sourceNode.value.connect(audioContext.destination)

  // Start playback from offset
  //   startTime.value = audioContext.currentTime
  sourceNode.value.start()
}

const togglePlay = async () => {
  const audioContext = await initializeAudioContext()

  if (isPlaying.value) {
    // Pause: store current position and disconnect source
    offset.value += audioContext.currentTime - startTime.value
    sourceNode.value.disconnect()
    sourceNode.value.stop()
    isPlaying.value = false
  } else {
    // Play: create new source and start from stored offset
    createAndStartSource(audioContext)
    isPlaying.value = true
  }
}

initializeAudioContext()

// Clean up on component unmount
onBeforeUnmount(() => {
  if (sourceNode.value) {
    sourceNode.value.disconnect()
  }
  //   if (audioContext.value) {
  //     audioContext.value.close()
  //   }
})
</script>
<template>
  <div class="audio-player">
    <button @click="togglePlay" class="play-button">
      {{ isPlaying ? 'Pause' : 'Play' }}
    </button>
  </div>
</template>

<style scoped>
.audio-player {
  padding: 1rem;
  height: 100vh;
}

.play-button {
  padding: 0.5rem 1rem;
  background-color: #222222;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  width: 100%;
  height: 100%;
}

.play-button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}
</style>
