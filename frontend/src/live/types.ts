export type Hop = {
  frequencies: Float32Array
  magnitudes: Float32Array
}

export type HopMessage = {
  type: 'hop'
  data: Hop
}

export type ProcessorMessage = HopMessage

export type InitializeMessage = {
  type: 'initialize'
  wasmBytes: ArrayBuffer
}

export type NodeMessage = InitializeMessage

export interface SineWaveSpeechProcessorOptions {
  // do we still need this? used to have hopSize here
}

export interface SineWaveSpeechNodeOptions extends AudioWorkletNodeOptions {
  // processorOptions is required but we can't enforce that with TypeScript
  // because we're extending AudioWorkletNodeOptions
  processorOptions?: SineWaveSpeechProcessorOptions
  onHop: (hop: Hop) => void
}
