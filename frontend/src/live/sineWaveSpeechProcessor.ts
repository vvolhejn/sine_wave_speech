import init, { SineWaveSpeechConverter } from '../wasm_realtime_sws/wasm_audio.js'
import { ProcessorMessage, SineWaveSpeechNodeOptions } from './types.js'

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

  // Dummy values, these get overwritten in the first process() call
  lastHopSize: number = 0
  lastNWaves: number = 0
  lastFrequencies: Float32Array = new Float32Array()
  lastMagnitudes: Float32Array = new Float32Array()
  lastPhases: Float32Array = new Float32Array()

  bufferToPlay: Float32Array
  bufferToProcess: Float32Array

  constructor(options?: SineWaveSpeechNodeOptions) {
    super(options)

    if (!options || !options.processorOptions) {
      throw new Error('Expected options.processorOptions to be defined')
    }

    // Since the hop size might be larger than the block size, we need to buffer the audio
    // until we have enough to process. After processing, we make a buffer and then play it,
    // again over multiple blocks.
    this.bufferToPlay = new Float32Array()
    this.bufferToProcess = new Float32Array()

    this.port.onmessage = (event) => this.onmessage(event.data)
  }

  static get parameterDescriptors(): AudioParamDescriptor[] {
    return [
      {
        name: 'frequencyQuantizationStrength',
        defaultValue: 0.0,
        minValue: 0,
        maxValue: 3,
      },
      {
        name: 'hopSizeMultiplier',
        defaultValue: 2,
        minValue: 1,
        maxValue: 16,
      },
      {
        name: 'nWaves',
        defaultValue: 4,
        minValue: 1,
        maxValue: 16,
      },
      {
        name: 'gainDb',
        defaultValue: 0,
        minValue: -18,
        maxValue: 18,
      },
    ]
  }

  onmessage(event: MessageEvent) {
    if (event.type === 'initialize') {
      init(WebAssembly.compile((event as any).wasmBytes)).then(() => {
        // n_waves and hop_size get overwritten in the first process() call
        // (perhaps we shouldn't even have them here)
        this.converter = SineWaveSpeechConverter.new(4, BLOCK_SIZE, sampleRate)
      })
    } else {
      throw new Error('Unknown message type: ' + event.type)
    }
  }

  process(
    inputList: Float32Array[][],
    outputList: Float32Array[][],
    parameters: Record<string, Float32Array>
  ) {
    if (!canProcess(inputList, outputList)) {
      return true
    }

    // It seems that even if there are two channels, we get stereo output even
    // though we're only writing one channel - why?
    const inputAudio = inputList[0][0]
    const outputAudio = outputList[0][0]

    if (this.converter === null) {
      // If we're still waiting for the converter to be initialized, just pass through the audio
      outputAudio.set(inputAudio)
      return true
    }

    const hopSize = BLOCK_SIZE * parameters.hopSizeMultiplier[0]
    const hopSizeChanged = hopSize !== this.lastHopSize
    if (hopSizeChanged) {
      this.lastHopSize = hopSize
      this.converter.hop_size = hopSize
      // Keeping the buffers from the previous hop size would lead to weird edge cases
      this.bufferToProcess = new Float32Array()
      this.bufferToPlay = new Float32Array()
    }

    const nWaves = parameters.nWaves[0]
    if (nWaves !== this.lastNWaves) {
      this.lastFrequencies = new Float32Array(nWaves)
      this.lastMagnitudes = new Float32Array(nWaves)
      this.lastPhases = new Float32Array(nWaves)
      this.bufferToProcess = new Float32Array()
      this.bufferToPlay = new Float32Array()
      this.converter.n_waves = nWaves
      this.lastNWaves = nWaves
    }

    // Not very efficient because we're creating a new array, but it's fine for now
    this.bufferToProcess = concatFloat32Arrays([this.bufferToProcess, inputAudio])

    if (this.bufferToProcess.length >= hopSize) {
      if (this.bufferToProcess.length > hopSize) {
        throw new Error(
          'Buffer to process is too long. ' +
            "This means the algorithm's hop size isn't a multiple of the block size"
        )
      }

      const fm = this.converter.get_frequencies_and_magnitudes(this.bufferToProcess)
      let frequencies = fm.slice(0, fm.length / 2)
      let magnitudes = fm.slice(fm.length / 2)

      const frequencyQuantizationStrength = parameters.frequencyQuantizationStrength[0]
      frequencies = this.converter.quantize_frequencies_continuous(
        frequencies,
        frequencyQuantizationStrength
      )

      magnitudes = addGain(magnitudes, parameters.gainDb[0])

      if (frequencies.length !== nWaves) {
        throw new Error(`Expected ${nWaves} frequencies, got ${frequencies.length}`)
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
      const audio = converted.slice(0, hopSize)
      const lastPhases = converted.slice(hopSize)

      if (this.bufferToPlay.length > 0) {
        throw new Error('Buffer to play is not empty')
      }
      this.bufferToPlay = audio

      this.lastFrequencies = frequencies
      this.lastMagnitudes = magnitudes
      this.lastPhases = lastPhases
      this.bufferToProcess = new Float32Array()

      this.postMessage({
        type: 'hop',
        data: {
          frequencies: frequencies,
          magnitudes: magnitudes,
        },
      })
    }

    if (this.bufferToPlay.length >= BLOCK_SIZE) {
      outputAudio.set(this.bufferToPlay.slice(0, BLOCK_SIZE))
      this.bufferToPlay = this.bufferToPlay.slice(BLOCK_SIZE)
    } else {
      // This happens at the beginning of the audio, before we've processed the first hop
    }

    return true
  }

  /** A typed version of this.port.postMessage() */
  private postMessage(message: ProcessorMessage) {
    this.port.postMessage(message)
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

  if (inputChannels.length === 0) {
    // Silently skip - this happens if we're not getting any input.
    // That happens when we're processing an audio file and it ends.
    return false
  }

  if (inputChannels[0].length !== BLOCK_SIZE) {
    throw new Error(`Expected ${BLOCK_SIZE} samples, got ${inputChannels[0].length}`)
  }

  return true
}

const addGain = (magnitudes: Float32Array, gainDb: number): Float32Array => {
  const gain = Math.pow(10, gainDb / 20)
  return magnitudes.map((m) => m * gain)
}

registerProcessor('sine-wave-speech-processor', SineWaveSpeechProcessor)
