<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted } from 'vue'
// temporarily disabled for debugging
import originalAudio from '../assets/explanation-1.mp3'
import swsData from '../assets/explanation-1.json'
// import originalAudio from '../assets/sentence-sine-wave.wav'
// import swsData from '../assets/sentence-sine-wave.json'
import { usePlaybackStore } from '../stores/playbackStore'
import { setUpSineWaveSpeechAudio } from '../audio'
import _ from 'lodash'
import Page from './Page.vue'

// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784
// https://coolors.co/1e152a-4e6766-5ab1bb-a5c882-f7dd72

// const audioContext = new window.AudioContext()
const playbackStore = usePlaybackStore()
playbackStore.setSwsData(swsData)

const handleScroll = () => {
  const maxScroll = document.body.scrollHeight - window.innerHeight
  const scrollFraction = window.scrollY / maxScroll
  playbackStore.setScrollFraction(scrollFraction)
}

let throttledHandleScroll: EventListener | null = null

onMounted(() => {
  setUpSineWaveSpeechAudio(originalAudio)

  // throttle to 60 FPS (are we actually throttling anything?)
  throttledHandleScroll = _.throttle(handleScroll, 1000 / 60)
  window.addEventListener('scroll', throttledHandleScroll)
})

onBeforeUnmount(() => {
  if (throttledHandleScroll) {
    window.removeEventListener('scroll', throttledHandleScroll)
  }
})

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

const showLowerHeader = computed(() => {
  // Mention this dependency so that this computed is re-run when animationTime changes.
  // We actually care about the audioContext time, but that's not reactive.
  playbackStore.animationTime

  const ALLOW_SCROLL_FROM_SEC = 42.0
  return playbackStore.audioContext.currentTime > ALLOW_SCROLL_FROM_SEC
  // return !['init', 'basics', 'basics2'].includes(messageStore.currentMessageKey)
})
</script>

<template>
  <Page :on-click="onClick" :is-original="false">Sine Wave Speech</Page>
  <Page :on-click="onClick" :is-original="true" v-if="showLowerHeader">Original</Page>
</template>
