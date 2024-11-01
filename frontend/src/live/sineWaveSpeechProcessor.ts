import init, { SineWaveSpeechConverter } from '../wasm_realtime_sws/wasm_audio.js'

class SineWaveSpeechProcessor extends AudioWorkletProcessor {
  converter: SineWaveSpeechConverter | null = null
  nWaves: number = 4
  // currently will break for anything other than 128 because that's the buffer size that
  // audio worklet uses
  hopSize: number = 128
  lastFrequencies: Float32Array
  lastMagnitudes: Float32Array
  lastPhases: Float32Array

  constructor() {
    super()
    this.lastFrequencies = new Float32Array(this.nWaves)
    this.lastMagnitudes = new Float32Array(this.nWaves)
    this.lastPhases = new Float32Array(this.nWaves)

    this.port.onmessage = (event) => this.onmessage(event.data)
  }

  onmessage(event: MessageEvent) {
    if (event.type === 'initialize') {
      init(WebAssembly.compile((event as any).wasmBytes)).then(() => {
        this.converter = SineWaveSpeechConverter.new(this.nWaves, this.hopSize)
        console.log('Initialized')
        this.port.postMessage({ type: 'initialized' })
      })
    } else {
      throw new Error('Unknown message type: ' + event.type)
    }
  }

  process(
    inputList: Float32Array[][],
    outputList: Float32Array[][],
    _parameters: Record<string, Float32Array>
  ) {
    if (!canProcess(inputList, outputList)) {
      return true
    }
    const inputAudio = inputList[0][0]
    const outputAudio = outputList[0][0]

    if (this.converter === null) {
      outputAudio.set(inputAudio)
    } else {
      const fm = this.converter.get_frequencies_and_magnitudes(inputAudio)
      const frequencies = fm.slice(0, fm.length / 2)
      const magnitudes = fm.slice(fm.length / 2)

      if (frequencies.length !== this.nWaves) {
        throw new Error(
          `Expected ${this.nWaves} frequencies, got ${frequencies.length}`
        )
      }

      const combinedFrequencies = new Float32Array(this.nWaves * 2)
      const combinedMagnitudes = new Float32Array(this.nWaves * 2)
      combinedFrequencies.set(this.lastFrequencies)
      combinedFrequencies.set(frequencies, this.nWaves)
      combinedMagnitudes.set(this.lastMagnitudes)
      combinedMagnitudes.set(magnitudes, this.nWaves)

      const converted = this.converter.synthesize(
        combinedFrequencies,
        combinedMagnitudes,
        // The phase needs to be passed in from the previous frame
        // so that the sine wave can continue from where it left off
        this.lastPhases
      )
      const audio = converted.slice(0, this.hopSize + 1)
      const lastPhases = converted.slice(this.hopSize + 1)

      outputAudio.set(audio.slice(1))

      this.lastFrequencies = frequencies
      this.lastMagnitudes = magnitudes
      this.lastPhases = lastPhases
    }

    return true
  }
}

const canProcess = (inputList: Float32Array[][], outputList: Float32Array[][]) => {
  if (inputList.length !== 1) {
    throw new Error('Expected exactly 1 input')
  }
  if (outputList.length !== 1) {
    throw new Error('Expected exactly 1 output')
  }
  const inputChannels = inputList[0]
  const outputChannels = outputList[0]

  if (inputChannels.length === 0) {
    // Silently skip - this happens if we're not getting any input.
    // That happens when we're processing an audio file and it ends.
    return false
  }

  if (inputChannels.length !== 1) {
    throw new Error('Expected exactly 1 input channel, got ' + inputChannels.length)
  }
  if (outputChannels.length !== 1) {
    throw new Error('Expected exactly 1 output channel')
  }

  return true
}

registerProcessor('sine-wave-speech-processor', SineWaveSpeechProcessor)
