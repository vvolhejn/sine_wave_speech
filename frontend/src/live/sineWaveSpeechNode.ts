export default class SineWaveSpeechNode extends AudioWorkletNode {
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

    this.port.postMessage({
      type: 'initialize',
      wasmBytes,
    })
  }

  // Handle an uncaught exception thrown in the PitchProcessor.
  onprocessorerror = (event: Event) => {
    console.log(`An error from AudioWorkletProcessor.process() occurred: ${event}`)
  }
}
