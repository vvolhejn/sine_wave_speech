<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import originalAudio from '../assets/explanation-1.mp3'
import swsData from '../assets/explanation-1.json'
import { usePlaybackStore } from '../stores/playbackStore'
import { setUpSineWaveSpeechAudio } from '../audio'
import _ from 'lodash'
import Page from './Page.vue'
import { useMessageStore } from '../stores/messageStore'

// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784
// https://coolors.co/1e152a-4e6766-5ab1bb-a5c882-f7dd72

// const audioContext = new window.AudioContext()
const messageStore = useMessageStore()
const playbackStore = usePlaybackStore()
playbackStore.setSwsData(swsData)

// This gets set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)

const handleScroll = () => {
  const maxScroll = document.body.scrollHeight - window.innerHeight
  const scrollFraction = window.scrollY / maxScroll
  playbackStore.setScrollFraction(scrollFraction)
  messageStore.setScrollFraction(scrollFraction)
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
    messageStore.onMessageClick()
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
  <audio :src="originalAudio" ref="audioElement" @ended="onAudioEnded"></audio>
  <Page :on-click="onClick" :is-original="false">Sine Wave Speech</Page>
  <Page :on-click="onClick" :is-original="true" v-if="showLowerHeader">Original</Page>
</template>
