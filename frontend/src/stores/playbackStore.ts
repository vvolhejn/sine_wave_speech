import { defineStore } from 'pinia'
import { computed, ref } from 'vue'
import { SwsData } from '../types'
import { playSineWaveSpeechAudio } from '../audio'
import * as d3 from 'd3'

const smoothe = (arr: number[], windowLength: number) => {
  const result = new Array<number>(arr.length)
  for (let i = 0; i < arr.length; i++) {
    const start = Math.max(0, i - windowLength)
    const end = i + 1
    result[i] = arr.slice(start, end).reduce((a, b) => a + b, 0) / (end - start)
  }
  return result
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

  /** Note that `smoothedFrequencies` has transposed axes compared to `frequencies`.
   * So `smoothedFrequencies[i][j]` is the smoothed frequency of the i-th sine wave
   * at timestep j.
   */
  const smoothedFrequencies = computed(() => {
    if (!swsData.value) return null
    const parts = []
    for (let i = 0; i < swsData.value.frequencies[0].length; i++) {
      parts.push(
        smoothe(
          swsData.value.frequencies.map((x) => x[i]),
          30
        )
      )
    }

    return parts
  })

  /** See note above smoothedFrequencies - also applies here. */
  const smoothedMagnitudes = computed(() => {
    if (!swsData.value) return null
    const parts = []
    for (let i = 0; i < swsData.value.magnitudes[0].length; i++) {
      parts.push(
        smoothe(
          swsData.value.magnitudes.map((x) => x[i]),
          30
        )
      )
    }
    return parts
  })

  const animationTime = ref(0)
  const updateAnimationTime = () => {
    if (!isPlaying.value) {
      // Make the waves move even when no audio is playing
      animationTime.value = d3.now() / 1000
    } else {
      animationTime.value = audioContext.currentTime
    }
  }

  const swsIndex = computed(() => {
    if (!swsData.value) return null
    if (!isPlaying.value) return null

    const secondsPerTimestep = swsData.value.hopSize / swsData.value.sr

    let index = Math.floor((animationTime.value - startTime.value) / secondsPerTimestep)
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

    startTime.value = audioContext.currentTime
    isPlaying.value = true
    playSineWaveSpeechAudio()
  }

  return {
    audioContext,
    isPlaying,
    setIsPlaying,
    startTime,
    swsData,
    setSwsData,
    smoothedFrequencies,
    smoothedMagnitudes,
    animationTime,
    updateAnimationTime,
    swsIndex,
    playSineWaveSpeech,
  }
})
