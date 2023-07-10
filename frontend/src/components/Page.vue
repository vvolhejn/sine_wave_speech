<script setup lang="ts">
import { useMessageStore } from '../stores/messageStore'
import { usePlaybackStore } from '../stores/playbackStore'

const debug = false

const props = defineProps<{ onClick?: () => void; isOriginal: boolean }>()

const playbackStore = usePlaybackStore()
const messageStore = useMessageStore()
</script>
<template>
  <div class="flex flex-col items-center min-h-screen font-header text-center">
    <button @click="props.onClick">
      <h1 class="text-8xl italic mt-[20vh] md:mt-[35vh]">
        <slot />
      </h1>
      <template v-if="debug">
        <p>animationTime {{ playbackStore.animationTime.toFixed(2) }}</p>
        <p>
          audioContext.currentTime
          {{ playbackStore.audioContext.currentTime.toFixed(2) }}
        </p>
        <p>isPlaying {{ playbackStore.isPlaying }}</p>
        <p>startTime {{ playbackStore.startTime }}</p>
      </template>
    </button>
    <div class="text-3xl mt-20 p-4 max-w-xl font-body" @click="props.onClick">
      {{ messageStore.currentMessage }}
    </div>
  </div>
</template>
