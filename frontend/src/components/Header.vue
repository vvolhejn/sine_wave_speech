<script setup lang="ts">
import { usePlaybackStore } from '../stores/playbackStore'

const debug = false

const props = defineProps<{ onClick?: () => void }>()

const playbackStore = usePlaybackStore()
</script>
<template>
  <div class="flex flex-col min-h-screen justify-center">
    <button @click="props.onClick">
      <h1 class="text-8xl text-center mix-blend-difference font-[Playfair] italic z-30">
        <slot />
      </h1>
      <template v-if="debug">
        <p>animationTime {{ playbackStore.animationTime.toFixed(2) }}</p>
        <p>isPlaying {{ playbackStore.isPlaying }}</p>
        <p>startTime {{ playbackStore.startTime }}</p>
      </template>
    </button>
  </div>
</template>
