import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
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

type MessageKey =
  | 'init'
  | 'basics'
  | 'basics2'
  | 'scrollDown'
  | 'understanding'
  | 'understanding2'
  | 'end'

type Message = {
  text: string
  bottomText?: string
  goTo: MessageKey
  timeoutMs?: number
  waitForTimeout?: boolean
}

const TOP_SCROLL_THRESHOLD = 0.1
const BOTTOM_SCROLL_THRESHOLD = 0.9

export const useMessageStore = defineStore('message', () => {
  const currentMessageKey = ref<MessageKey>('init')

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

  const setMessageTimeout = () => {
    const timeout = currentMessage.value.timeoutMs
    if (timeout) {
      setTimeout(() => {
        setMessage(currentMessage.value.goTo)
      }, timeout)
    }
  }

  const setMessage = (key: MessageKey) => {
    currentMessageKey.value = key

    const timeout = currentMessage.value.timeoutMs
    if (timeout && !currentMessage.value.waitForTimeout) {
      setMessageTimeout()
    }
  }

  const onMessageClick = () => {
    if (currentMessageKey.value === 'init') {
      setMessage('basics')
    }
  }

  const getScrollRegion = (scrollFraction: number) => {
    if (scrollFraction < TOP_SCROLL_THRESHOLD) {
      return 'top'
    }
    if (scrollFraction > BOTTOM_SCROLL_THRESHOLD) {
      return 'bottom'
    }
    return 'middle'
  }

  const scrollFraction = ref(0)
  const setScrollFraction = (value: number) => {
    const oldScrollRegion = getScrollRegion(scrollFraction.value)
    const newScrollRegion = getScrollRegion(value)

    if (oldScrollRegion !== 'bottom' && newScrollRegion === 'bottom') {
      console.log('scroll down')
      if (currentMessageKey.value === 'scrollDown') {
        setMessageTimeout()
      }
    }

    scrollFraction.value = value
  }

  return {
    currentMessageKey,
    currentMessage,
    onMessageClick,
    setScrollFraction,
  }
})
