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
      return 'Pause'
    case PlaybackState.Recording:
      return 'Pause'
  }
}
</script>
<template>
  <div class="grid grid-cols-2 content-center justify-items-center">
    <p class="self-center">Real-time</p>
    <div>
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
    <p class="self-center">Recording</p>
    <div>
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
      >
        {{ getPlayPauseText(playbackState) }}
      </Button>
    </div>
  </div>
</template>
