import { usePlaybackStore } from './stores/playbackStore'

export const playSineWaveSpeechAudio = () => {
  const playbackStore = usePlaybackStore()
  const audioContext = playbackStore.audioContext
  const swsData = playbackStore.swsData
  const time = audioContext.currentTime

  if (!swsData) return

  const oscillators = new Array<OscillatorNode>()
  const gains = new Array<GainNode>()
  const nWaves = swsData.frequencies[0].length
  const nTimesteps = swsData.frequencies.length

  for (let i = 0; i < nWaves; i++) {
    const osc = new OscillatorNode(audioContext, {
      frequency: swsData.frequencies[0][i],
      type: 'sine',
    })
    // swsData.magnitudes[0][i]
    const gain = new GainNode(audioContext, { gain: 0 })
    osc.connect(gain).connect(audioContext.destination)
    oscillators.push(osc)
    gains.push(gain)
  }

  // Check if context is in suspended state (autoplay policy)
  if (audioContext.state === 'suspended') {
    audioContext.resume()
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
        swsData.magnitudes[t][i],
        time + t * secondsPerTimestep
      )
    }
  }

  oscillators.forEach((oscillator) => {
    oscillator.start(time)
    oscillator.stop(time + nTimesteps * secondsPerTimestep)
  })
}
