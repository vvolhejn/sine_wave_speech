import { Hop, NodeMessage, ProcessorMessage, SineWaveSpeechNodeOptions } from './types'

export default class SineWaveSpeechNode extends AudioWorkletNode {
  onHop: (hop: Hop) => void

  /**
   * Initialize the Audio processor by sending the fetched WebAssembly module to
   * the processor worklet.
   */
  constructor(
    context: BaseAudioContext,
    name: string,
    wasmBytes: ArrayBuffer,
    options: SineWaveSpeechNodeOptions
  ) {
    super(context, name, options)

    this.postMessage({
      type: 'initialize',
      wasmBytes,
    })
    this.port.onmessage = (event) => this.onmessage(event.data)
    this.onHop = options.onHop
  }

  // Handle an uncaught exception thrown in the PitchProcessor.
  onprocessorerror = (event: Event) => {
    console.log(`An error from AudioWorkletProcessor.process() occurred: ${event}`)
  }

  onmessage(event: ProcessorMessage) {
    if (event.type === 'hop') {
      this.onHop(event.data)
    } else {
      throw new Error('Unknown message type: ' + event.type)
    }
  }

  /** A typed version of this.port.postMessage() */
  private postMessage(message: NodeMessage) {
    this.port.postMessage(message)
  }
}
