<script setup lang="ts">
import { onBeforeUnmount, onMounted } from 'vue'

import { PlaybackState } from '../types'
import Button from './Button.vue'

const playbackState = defineModel<PlaybackState>({ required: true })

const onPlayPauseButtonClick = async () => {
  switch (playbackState.value) {
    case PlaybackState.Stopped:
    case PlaybackState.PlayingRealtime:
      playbackState.value = PlaybackState.PlayingRecorded
      break
    case PlaybackState.PlayingRecorded:
      playbackState.value = PlaybackState.Stopped
      break
  }
}

const onRealtimeButtonClick = async () => {
  if (playbackState.value === PlaybackState.PlayingRealtime) {
    playbackState.value = PlaybackState.Stopped
  } else {
    playbackState.value = PlaybackState.PlayingRealtime
  }
}

const getPlayPauseText = (state: PlaybackState) => {
  switch (state) {
    case PlaybackState.Stopped:
      return 'Play'
    case PlaybackState.PlayingRecorded:
      return 'Pause'
    case PlaybackState.PlayingRealtime:
      return 'Play'
    case PlaybackState.Recording:
      return 'Pause'
  }
}

const handleKeyPress = (event: any) => {
  // pause/play with space
  if (event.key === ' ') {
    onPlayPauseButtonClick()
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeyPress)
})
onBeforeUnmount(() => {
  document.removeEventListener('keydown', handleKeyPress)
})
</script>
<template>
  <div class="grid grid-cols-2 gap-1">
    <p class="self-center">Record, then convert</p>
    <div class="flex flex-row w-full">
      <Button
        :disabled="playbackState === PlaybackState.Recording"
        @click="
          () => {
            playbackState = PlaybackState.Recording
          }
        "
        custom-class="grow-0"
      >
        Record
      </Button>
      <Button
        :disabled="playbackState === PlaybackState.Recording"
        @click="onPlayPauseButtonClick"
        custom-class="grow"
      >
        {{ getPlayPauseText(playbackState) }}
      </Button>
    </div>

    <p class="self-center">Convert in real time</p>
    <div class="flex flex-row w-full">
      <Button
        :disabled="playbackState === PlaybackState.Recording"
        @click="onRealtimeButtonClick"
        custom-class="grow"
      >
        {{
          playbackState !== PlaybackState.PlayingRealtime
            ? 'Start real-time'
            : 'Stop real-time'
        }}
      </Button>
    </div>
  </div>
</template>
