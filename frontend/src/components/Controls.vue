<script setup lang="ts">
import { onMounted, ref } from 'vue'
import sentence from '../assets/sentence-original.wav'
import swsData from '../assets/virgil-abloh-clip.json'
import { usePlaybackStore } from '../stores/playbackStore'
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
  <audio :src="sentence" ref="audioElement" @ended="onAudioEnded"></audio>
  <div class="flex flex-col min-h-screen justify-center">
    <button @click="playbackStore.playSineWaveSpeech">
      <h1 class="text-8xl text-center mix-blend-difference font-[Playfair] italic">
        Sine Wave Speech
      </h1>
    </button>
    <!-- <button
      data-playing="false"
      role="switch"
      aria-checked="false"
      @click="onClick"
      ref="playButton"
    >
      <span>Play original</span>
    </button> -->
  </div>
</template>
