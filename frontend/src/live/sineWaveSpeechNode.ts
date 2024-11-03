import { ref } from 'vue'

import { Hop, NodeMessage, ProcessorMessage } from './types'

export default class SineWaveSpeechNode extends AudioWorkletNode {
  hops = ref<Hop[]>([])

  /**
   * Initialize the Audio processor by sending the fetched WebAssembly module to
   * the processor worklet.
   */
  constructor(
    context: BaseAudioContext,
    name: string,
    wasmBytes: ArrayBuffer,
    options?: AudioWorkletNodeOptions
  ) {
    super(context, name, options)

    this.postMessage({
      type: 'initialize',
      wasmBytes,
    })
    this.port.onmessage = (event) => this.onmessage(event.data)
  }

  // Handle an uncaught exception thrown in the PitchProcessor.
  onprocessorerror = (event: Event) => {
    console.log(`An error from AudioWorkletProcessor.process() occurred: ${event}`)
  }

  onmessage(event: ProcessorMessage) {
    if (event.type === 'hop') {
      this.hops.value.push(event.data)
      // TODO: is this the best place to manage the max length?
      if (this.hops.value.length > 200) {
        this.hops.value.shift()
      }
    } else {
      throw new Error('Unknown message type: ' + event.type)
    }
  }

  /** A typed version of this.port.postMessage() */
  private postMessage(message: NodeMessage) {
    this.port.postMessage(message)
  }
}
