<script setup lang="ts">
import { computed } from 'vue'
import { useMessageStore } from '../stores/messageStore'
import { usePlaybackStore } from '../stores/playbackStore'

const debug = false

const props = defineProps<{ onClick?: () => void; isOriginal: boolean }>()

const playbackStore = usePlaybackStore()
const messageStore = useMessageStore()

const message = computed(() => {
  if (props.isOriginal) {
    return messageStore.currentMessage.bottomText || messageStore.currentMessage.text
  } else {
    return messageStore.currentMessage.text
  }
})
</script>
<template>
  <div class="flex flex-col items-center min-h-screen font-header text-center">
    <button @click="props.onClick">
      <h1 class="text-8xl italic mt-[35vh]">
        <slot />
      </h1>
      <template v-if="debug">
        <p>animationTime {{ playbackStore.animationTime.toFixed(2) }}</p>
        <p>isPlaying {{ playbackStore.isPlaying }}</p>
        <p>startTime {{ playbackStore.startTime }}</p>
      </template>
    </button>
    <div class="text-3xl mt-20 max-w-xl font-body" @click="props.onClick">
      {{ message }}
    </div>
  </div>
</template>
