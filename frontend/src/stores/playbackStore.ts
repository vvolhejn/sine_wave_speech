import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { SwsData } from '../types'
import * as d3 from 'd3'

const secondsNow = () => {
  return d3.now() / 1000
}

export const usePlaybackStore = defineStore('playback', () => {
  const audioContext = new AudioContext()

  const audioElement = ref<HTMLAudioElement | null>(null)
  const setAudioElement = (element: HTMLAudioElement) => {
    audioElement.value = element
  }

  const isPlaying = ref(false)
  const setIsPlaying = (value: boolean) => {
    isPlaying.value = value
  }

  const startTime = ref(0)
  const swsData = ref<SwsData | null>(null)

  const setSwsData = (data: SwsData) => {
    swsData.value = data
  }

  const animationTime = ref(0)
  const updateAnimationTime = () => {
    if (!audioSetupDoneAt.value) {
      // setting up the audio is computationally intensive, so to avoid visible stutter,
      // disable the movement before we are done
      return 0
    }
    if (!isPlaying.value) {
      // Make the waves move even when no audio is playing
      animationTime.value = secondsNow() - audioSetupDoneAt.value
    } else {
      animationTime.value = audioContext.currentTime + startTime.value
    }
  }

  const setScrollFraction = (value: number) => {
    if (!audioNodes.value) return

    audioNodes.value.originalGain.gain.value = value
    audioNodes.value.swsGain.gain.value = 1 - value
  }

  const swsIndex = computed(() => {
    if (!swsData.value) return null
    if (!isPlaying.value) return null

    const secondsPerTimestep = swsData.value.hopSize / swsData.value.sr

    // Pretend we're using animationTime to trick Vue into thinking we depend on it
    let index = Math.floor(
      (animationTime.value * 0.0 + audioContext.currentTime) / secondsPerTimestep
    )

    if (index < 0) {
      index = 0
    }
    if (index >= swsData.value.frequencies.length) {
      index = swsData.value.frequencies.length - 1
    }

    return index
  })

  const playSineWaveSpeech = () => {
    if (isPlaying.value) return

    startTime.value = animationTime.value - audioContext.currentTime
    isPlaying.value = true

    // Check if context is in suspended state (autoplay policy)
    if (audioContext.state === 'suspended') {
      audioContext.resume()
      audioElement.value?.play()
    }
  }

  const audioSetupDoneAt = ref<number | null>(null)
  const audioNodes = ref<{
    originalGain: GainNode
    swsGain: GainNode
    analyser: AnalyserNode
  } | null>(null)

  const onAudioSetupDone = (
    originalGain: GainNode,
    swsGain: GainNode,
    analyser: AnalyserNode
  ) => {
    audioSetupDoneAt.value = secondsNow()
    audioNodes.value = { originalGain, swsGain, analyser }
  }

  return {
    audioContext,
    audioElement,
    setAudioElement,
    isPlaying,
    setIsPlaying,
    startTime,
    swsData,
    setSwsData,
    animationTime,
    updateAnimationTime,
    setScrollFraction,
    swsIndex,
    playSineWaveSpeech,
    audioSetupDoneAt,
    onAudioSetupDone,
    audioNodes,
  }
})
