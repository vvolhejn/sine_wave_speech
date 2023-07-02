import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

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

const MESSAGES: { [key in MessageKey]: Message } = {
  init: { text: 'Click to play audio.', goTo: 'basics' },
  basics: {
    text: "It might not sound like it, but what you're hearing is a person speaking. ",
    goTo: 'basics2',
    timeoutMs: 5000,
  },
  basics2: {
    text: 'The sound has been simplified to just four sine waves.',
    goTo: 'scrollDown',
    timeoutMs: 5000,
  },
  scrollDown: {
    text: 'Scroll down to hear what the original sounds like.',
    bottomText: 'Try going back and forth between the two.',
    goTo: 'understanding',
    timeoutMs: 5000,
    waitForTimeout: true,
  },
  understanding: {
    text: 'After a while, you should be able to understand sine wave speech.',
    goTo: 'understanding2',
    timeoutMs: 10000,
  },
  understanding2: {
    text: 'Think about how crazy that is.',
    bottomText: 'A minute ago it only sounded like weird whistles.',
    goTo: 'end',
    timeoutMs: 10000,
  },
  end: { text: 'Pretty cool, right?', goTo: 'init' },
}

const TOP_SCROLL_THRESHOLD = 0.1
const BOTTOM_SCROLL_THRESHOLD = 0.9

export const useMessageStore = defineStore('message', () => {
  const currentMessageKey = ref<MessageKey>('init')

  const currentMessage = computed(() => MESSAGES[currentMessageKey.value])

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
