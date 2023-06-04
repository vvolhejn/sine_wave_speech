import { defineStore } from 'pinia'
import { ref } from 'vue'
import { SwsData } from '../types'
import { playSineWaveSpeechAudio } from '../audio'

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

  const swsIndex = ref<number | null>(null)
  const getSwsIndex = (time?: number) => {
    if (!swsData.value) return null
    if (!time) time = audioContext.currentTime

    const secondsPerTimestep = swsData.value.hopSize / swsData.value.sr

    let index = Math.floor((time - startTime.value) / secondsPerTimestep)
    if (index < 0) {
      index = 0
    }
    if (index >= swsData.value.frequencies.length) {
      index = swsData.value.frequencies.length - 1
    }

    return index
  }
  const updateSwsIndex = () => {
    swsIndex.value = getSwsIndex()
    return swsIndex.value
  }

  const playSineWaveSpeech = () => {
    startTime.value = audioContext.currentTime
    playSineWaveSpeechAudio()
  }

  return {
    audioContext,
    isPlaying,
    setIsPlaying,
    startTime,
    swsData,
    setSwsData,
    swsIndex,
    updateSwsIndex,
    playSineWaveSpeech,
  }
})
