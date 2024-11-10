<script setup lang="ts">
import { PlaybackState } from '../types'
import Button from './Button.vue'

const playbackState = defineModel<PlaybackState>({ required: true })

const onPlayPause = async () => {
  switch (playbackState.value) {
    case PlaybackState.Stopped:
      playbackState.value = PlaybackState.PlayingRecorded
      break
    case PlaybackState.PlayingRecorded:
      playbackState.value = PlaybackState.Stopped
      break
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
</script>
<template>
  <div class="grid grid-cols-2 content-center justify-items-center gap-1">
    <div
      :class="[
        'col-span-2 grid grid-cols-2 rounded-lg p-3 w-full border',
        [PlaybackState.PlayingRecorded, PlaybackState.Recording].includes(playbackState)
          ? 'border-white'
          : 'border-transparent',
      ]"
    >
      <p class="self-center">Record, then convert</p>
      <div class="justify-self-center">
        <Button
          :disabled="playbackState === PlaybackState.Recording"
          @click="
            () => {
              playbackState = PlaybackState.Recording
            }
          "
        >
          Record
        </Button>
        <Button
          :disabled="playbackState === PlaybackState.Recording"
          @click="onPlayPause"
          custom-class="w-20"
        >
          {{ getPlayPauseText(playbackState) }}
        </Button>
      </div>
    </div>
    <div
      :class="[
        'col-span-2 grid grid-cols-2 rounded-lg p-3 w-full border',
        playbackState === PlaybackState.PlayingRealtime
          ? 'border-white'
          : 'border-transparent',
      ]"
    >
      <p class="self-center">Convert in real time</p>
      <div class="justify-self-center">
        <Button
          :disabled="false"
          @click="
            () => {
              playbackState = PlaybackState.PlayingRealtime
            }
          "
        >
          Start real-time
        </Button>
      </div>
    </div>
    <slot></slot>
  </div>
</template>
