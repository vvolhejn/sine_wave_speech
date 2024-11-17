<script setup lang="ts">
import _ from 'lodash'
import { onBeforeUnmount, onMounted } from 'vue'

import { playSineWaveSpeech, getAudioBuffer } from '../audio'
import { originalAudio, swsData } from '../dataFiles'
import { usePlaybackStore } from '../stores/playbackStore'
import Page from './Page.vue'

// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784
// https://coolors.co/1e152a-4e6766-5ab1bb-a5c882-f7dd72

// const audioContext = new window.AudioContext()
const playbackStore = usePlaybackStore()
playbackStore.setSwsData(swsData)

const handleScroll = () => {
  const maxScroll = Math.max(document.body.scrollHeight - window.innerHeight, 1)
  const scrollFraction = window.scrollY / maxScroll
  playbackStore.setScrollFraction(scrollFraction)
}

let throttledHandleScroll: EventListener | null = null

const setup = async () => {
  const audioBuffer = await getAudioBuffer(originalAudio)
  playbackStore.setOriginalAudioBuffer(audioBuffer)
  playSineWaveSpeech()
  playbackStore.audioContext.suspend()
}

onMounted(() => {
  setup()

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
</script>

<template>
  <div class="fixed bottom-0 right-0 m-4 sm:m-12 text-white text-2xl font-body">
    <p class="my-2">
      By
      <a href="https://vvolhejn.com" class="underline">Václav Volhejn</a>
    </p>
    <p class="my-2">
      <a href="/live/" class="underline">Try the effect live</a>
    </p>
    <p class="my-2">
      <a
        href="https://vvolhejn.github.io/2023/08/20/sinewavespeech-com.html"
        class="underline"
        >Blog post</a
      >
    </p>
  </div>
  <Page :on-click="onClick" :is-original="false">Sine Wave Speech</Page>
  <Page :on-click="onClick" :is-original="true" v-if="playbackStore.showLowerHeader"
    >Original</Page
  >
</template>
