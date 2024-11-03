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
