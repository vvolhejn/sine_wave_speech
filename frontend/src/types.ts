export type SwsData = {
  sr: number
  hopSize: number
  frequencies: number[][]
  magnitudes: number[][]
}

export type WaveConfig = {
  waveIndex: number
  yOffset: number
  color?: string
}
