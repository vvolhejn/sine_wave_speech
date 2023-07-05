import { defineStore } from 'pinia'
import { computed } from 'vue'
// @ts-ignore
import * as parser from 'subtitles-parser'
import rawSubtitles from '../assets/explanation-1.srt?raw'
import { usePlaybackStore } from './playbackStore'

type Subtitle = {
  id: string
  startTime: number
  endTime: number
  text: string
}

const subtitles: Subtitle[] = parser.fromSrt(rawSubtitles, true)

export const useMessageStore = defineStore('message', () => {
  // const currentMessage = computed(() => MESSAGES[currentMessageKey.value])

  const playbackStore = usePlaybackStore()

  const currentMessage = computed(() => {
    playbackStore.animationTime
    const timeMs = playbackStore.audioContext.currentTime * 1000

    if (timeMs === 0) {
      return 'Click to play audio.'
    }

    for (let i = 0; i < subtitles.length; i++) {
      const subtitle = subtitles[i]
      if (timeMs >= subtitle.startTime && timeMs <= subtitle.endTime) {
        return subtitle.text
      }
    }

    return ''
  })

  return {
    currentMessage,
  }
})
