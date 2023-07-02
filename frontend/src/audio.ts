import { usePlaybackStore } from './stores/playbackStore'

export const setUpSineWaveSpeechAudio = () => {
  const playbackStore = usePlaybackStore()
  const audioContext = playbackStore.audioContext
  const swsData = playbackStore.swsData
  const time = audioContext.currentTime

  if (!swsData) return
  if (!playbackStore.audioElement) return

  const originalAudio = audioContext.createMediaElementSource(
    playbackStore.audioElement
  )
  const originalGain = new GainNode(audioContext, { gain: 0.0 })
  const analyser = new AnalyserNode(audioContext, { fftSize: 2 ** 11 })
  originalAudio
    .connect(analyser)
    .connect(originalGain)
    .connect(audioContext.destination)

  const swsGain = new GainNode(audioContext, { gain: 1.0 })
  swsGain.connect(audioContext.destination)

  const oscillators = new Array<OscillatorNode>()
  const gains = new Array<GainNode>()
  const nWaves = swsData.frequencies[0].length
  const nTimesteps = swsData.frequencies.length

  for (let i = 0; i < nWaves; i++) {
    const osc = new OscillatorNode(audioContext, {
      frequency: swsData.frequencies[0][i],
      type: 'sine',
    })

    const gain = new GainNode(audioContext, { gain: 0 })
    osc.connect(gain).connect(swsGain)
    oscillators.push(osc)
    gains.push(gain)
  }

  const secondsPerTimestep = swsData.hopSize / swsData.sr

  for (let t = 0; t < nTimesteps; t++) {
    for (let i = 0; i < nWaves; i++) {
      const osc = oscillators[i]
      const gain = gains[i]
      osc.frequency.linearRampToValueAtTime(
        swsData.frequencies[t][i],
        time + t * secondsPerTimestep
      )
      gain.gain.linearRampToValueAtTime(
        swsData.magnitudes[t][i] * 0.1,
        time + t * secondsPerTimestep
      )
    }
  }

  oscillators.forEach((oscillator) => {
    oscillator.start(time)
    oscillator.stop(time + nTimesteps * secondsPerTimestep)
  })

  playbackStore.onAudioSetupDone(originalGain, swsGain, analyser)
}
