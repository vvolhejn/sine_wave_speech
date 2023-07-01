<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref } from 'vue'
import originalAudio from '../assets/ag-cook-clip.mp3'
import swsData from '../assets/ag-cook-clip.json'
import { usePlaybackStore } from '../stores/playbackStore'
import { setUpSineWaveSpeechAudio } from '../audio'
import _ from 'lodash'

// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784
// https://coolors.co/1e152a-4e6766-5ab1bb-a5c882-f7dd72

// const audioContext = new window.AudioContext()
const playbackStore = usePlaybackStore()
playbackStore.setSwsData(swsData)

// This gets set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)

const debug = false

const handleScroll = () => {
  const maxScroll = document.body.scrollHeight - window.innerHeight
  const scrollFraction = window.scrollY / maxScroll
  playbackStore.setScrollFraction(scrollFraction)
}

let throttledHandleScroll: EventListener | null = null

onMounted(() => {
  if (!audioElement.value) return

  playbackStore.setAudioElement(audioElement.value)

  // Without this little timeout, there is a white screen as the CPU-intensive operation
  // prevents the component from loading - even with async
  setTimeout(setUpSineWaveSpeechAudio, 100)

  // throttle to 60 FPS (are we actually throttling anything?)
  throttledHandleScroll = _.throttle(handleScroll, 1000 / 60)
  window.addEventListener('scroll', throttledHandleScroll)
})

onBeforeUnmount(() => {
  if (throttledHandleScroll) {
    window.removeEventListener('scroll', throttledHandleScroll)
  }
})

const onAudioEnded = () => {
  playbackStore.setIsPlaying(false)
}

const onClick = () => {
  // Play or pause track depending on state
  if (!playbackStore.isPlaying) {
    playbackStore.playSineWaveSpeech()
    playbackStore.setIsPlaying(true)
  } else {
    playbackStore.audioContext.suspend()
    playbackStore.setIsPlaying(false)
  }
}
</script>

<template>
  <audio :src="originalAudio" ref="audioElement" @ended="onAudioEnded"></audio>
  <div class="flex flex-col min-h-screen justify-center">
    <button @click="onClick">
      <h1 class="text-8xl text-center mix-blend-difference font-[Playfair] italic">
        Sine Wave Speech
      </h1>
      <template v-if="debug">
        <p>animationTime {{ playbackStore.animationTime.toFixed(2) }}</p>
        <p>isPlaying {{ playbackStore.isPlaying }}</p>
        <p>startTime {{ playbackStore.startTime }}</p>
      </template>
    </button>
  </div>
  <div class="flex flex-col min-h-screen justify-center">
    <h1 class="text-8xl text-center mix-blend-difference font-[Playfair] italic">
      Original
    </h1>
  </div>
</template>
