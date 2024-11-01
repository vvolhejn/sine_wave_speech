import init, { SineWaveSpeechConverter } from '../wasm_realtime_sws/wasm_audio.js'

// https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API/Using_AudioWorklet
// "By specification, each block of audio your process() function receives
// contains 128 frames (that is, 128 samples for each channel), but it is
// planned that this value will change in the future, and may in fact vary
// depending on circumstances, so you should always check the array's length
// rather than assuming a particular size."
// Here we do make the assumption but fail loudly if it's not true
const BLOCK_SIZE = 128

class SineWaveSpeechProcessor extends AudioWorkletProcessor {
  converter: SineWaveSpeechConverter | null = null
  nWaves: number = 4

  hopSize: number = BLOCK_SIZE * 2
  lastFrequencies: Float32Array
  lastMagnitudes: Float32Array
  lastPhases: Float32Array

  bufferToPlay: Float32Array
  bufferToProcess: Float32Array

  constructor() {
    super()
    this.lastFrequencies = new Float32Array(this.nWaves)
    this.lastMagnitudes = new Float32Array(this.nWaves)
    this.lastPhases = new Float32Array(this.nWaves)

    // Since the hop size might be larger than the block size, we need to buffer the audio
    // until we have enough to process. After processing, we make a buffer and then play it,
    // again over multiple blocks.
    this.bufferToPlay = new Float32Array()
    this.bufferToProcess = new Float32Array()

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
      // If we're still waiting for the converter to be initialized, just pass through the audio
      outputAudio.set(inputAudio)
      return true
    }

    // Not very efficient because we're creating a new array, but it's fine for now
    this.bufferToProcess = concatFloat32Arrays([this.bufferToProcess, inputAudio])

    if (this.bufferToProcess.length >= this.hopSize) {
      if (this.bufferToProcess.length > this.hopSize) {
        throw new Error(
          'Buffer to process is too long. ' +
            "This means the algorithm's hop size isn't a multiple of the block size"
        )
      }

      const fm = this.converter.get_frequencies_and_magnitudes(this.bufferToProcess)
      const frequencies = fm.slice(0, fm.length / 2)
      const magnitudes = fm.slice(fm.length / 2)

      if (frequencies.length !== this.nWaves) {
        throw new Error(
          `Expected ${this.nWaves} frequencies, got ${frequencies.length}`
        )
      }

      const combinedFrequencies = concatFloat32Arrays([
        this.lastFrequencies,
        frequencies,
      ])
      const combinedMagnitudes = concatFloat32Arrays([this.lastMagnitudes, magnitudes])

      const converted = this.converter.synthesize(
        combinedFrequencies,
        combinedMagnitudes,
        // The phase needs to be passed in from the previous frame
        // so that the sine wave can continue from where it left off
        this.lastPhases
      )
      const audio = converted.slice(0, this.hopSize)
      const lastPhases = converted.slice(this.hopSize)

      if (this.bufferToPlay.length > 0) {
        throw new Error('Buffer to play is not empty')
      }
      this.bufferToPlay = audio

      this.lastFrequencies = frequencies
      this.lastMagnitudes = magnitudes
      this.lastPhases = lastPhases
      this.bufferToProcess = new Float32Array()
    }

    if (this.bufferToPlay.length >= BLOCK_SIZE) {
      outputAudio.set(this.bufferToPlay.slice(0, BLOCK_SIZE))
      this.bufferToPlay = this.bufferToPlay.slice(BLOCK_SIZE)
    } else {
      // This happens at the beginning of the audio, before we've processed the first hop
    }

    return true
  }
}

const concatFloat32Arrays = (arrays: Float32Array[]) => {
  const totalLength = arrays.reduce((acc, arr) => acc + arr.length, 0)
  const result = new Float32Array(totalLength)
  let offset = 0
  for (const arr of arrays) {
    result.set(arr, offset)
    offset += arr.length
  }
  return result
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
  if (inputChannels[0].length !== BLOCK_SIZE) {
    throw new Error(`Expected ${BLOCK_SIZE} samples, got ${inputChannels[0].length}`)
  }

  return true
}

registerProcessor('sine-wave-speech-processor', SineWaveSpeechProcessor)
