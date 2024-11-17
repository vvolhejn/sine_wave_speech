import { defineStore } from 'pinia'
// @ts-ignore
import * as parser from 'subtitles-parser'
import { computed } from 'vue'

import { subtitles as rawSubtitles } from '../dataFiles'
import { usePlaybackStore } from './playbackStore'

type Subtitle = {
  id: string
  startTime: number
  endTime: number
  text: string
}

const subtitles: Subtitle[] = parser.fromSrt(rawSubtitles, true)

export const iOS = () => {
  return (
    [
      'iPad Simulator',
      'iPhone Simulator',
      'iPod Simulator',
      'iPad',
      'iPhone',
      'iPod',
    ].includes(navigator.platform) ||
    // iPad on iOS 13 detection
    (navigator.userAgent.includes('Mac') && 'ontouchend' in document)
  )
}

export const useMessageStore = defineStore('message', () => {
  // const currentMessage = computed(() => MESSAGES[currentMessageKey.value])

  const playbackStore = usePlaybackStore()

  const currentMessage = computed(() => {
    const timeMs = playbackStore.relativeAudioTime * 1000

    if (timeMs === 0) {
      return (
        'Click anywhere to play audio.' +
        (iOS() ? ' Make sure your iPhone is not in silent mode.' : '')
      )
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
