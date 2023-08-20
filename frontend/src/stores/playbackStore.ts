import * as d3 from 'd3'
import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

import { SwsData } from '../types'

const secondsNow = () => {
  return d3.now() / 1000
}

export const usePlaybackStore = defineStore('playback', () => {
  const audioContext = new AudioContext()

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

  const scrollFraction = ref(0)
  const setScrollFraction = (value: number) => {
    if (!audioNodes.value) return

    if (!showLowerHeader.value) {
      // If the page doesn't fit vertically when the lower header is off, we'd allow
      // scrolling - and therefore playing the original sound - even when we don't
      // want to.
      value = 0
    }

    audioNodes.value.originalGain.gain.value = value
    audioNodes.value.swsGain.gain.value = 1 - value
    scrollFraction.value = value
  }

  /**
   * When did the current loop start, in audioContext time (seconds)? This is used to
   * calculate the current index in the SWS data, because the audioContext time goes on
   * even after the first loop and doesn't restart to 0.
   */
  const loopStartTime = ref(0)

  /**
   * How far along in the audio playback are we, in seconds? Correctly resets when the
   * audio loops, unlike audioContext.currentTime.
   */
  const relativeAudioTime = computed(() => {
    // Pretend we're using animationTime to trick Vue into thinking we depend on it.
    // It doesn't notice when audioContext.currentTime changes, so we have to do this.
    animationTime.value

    return audioContext.currentTime - loopStartTime.value
  })

  const swsIndex = computed(() => {
    if (!swsData.value) return null
    if (!isPlaying.value) return null

    const secondsPerTimestep = swsData.value.hopSize / swsData.value.sr

    let index = Math.floor(relativeAudioTime.value / secondsPerTimestep)

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
    }
  }

  const originalAudioBuffer = ref<AudioBuffer | null>(null)
  const setOriginalAudioBuffer = (buffer: AudioBuffer) => {
    originalAudioBuffer.value = buffer
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
    if (audioSetupDoneAt.value == null) {
      audioSetupDoneAt.value = secondsNow()
    }
    loopStartTime.value = audioContext.currentTime
    audioNodes.value = { originalGain, swsGain, analyser }
  }

  const showLowerHeader = computed(() => {
    // Mention this dependency so that this computed is re-run when animationTime changes.
    // We actually care about the audioContext time, but that's not reactive.
    animationTime.value

    const ALLOW_SCROLL_FROM_SEC = 42.0
    return audioContext.currentTime >= ALLOW_SCROLL_FROM_SEC
  })

  return {
    audioContext,
    isPlaying,
    setIsPlaying,
    startTime,
    swsData,
    setSwsData,
    animationTime,
    updateAnimationTime,
    scrollFraction,
    setScrollFraction,
    relativeAudioTime,
    swsIndex,
    playSineWaveSpeech,
    originalAudioBuffer,
    setOriginalAudioBuffer,
    audioSetupDoneAt,
    onAudioSetupDone,
    audioNodes,
    showLowerHeader,
  }
})
