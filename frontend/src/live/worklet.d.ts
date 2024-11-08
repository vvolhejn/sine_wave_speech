// https://github.com/microsoft/TypeScript/issues/28308#issuecomment-650802278
// I also tried https://www.npmjs.com/package/@types/audioworklet
// but didn't manage to make it work
interface AudioWorkletProcessor {
  readonly port: MessagePort
  process(
    inputs: Float32Array[][],
    outputs: Float32Array[][],
    parameters: Record<string, Float32Array>
  ): boolean
}

interface AudioParamDescriptor {
  name: string
  defaultValue: number
  minValue: number
  maxValue: number
}

declare var AudioWorkletProcessor: {
  prototype: AudioWorkletProcessor
  new (options?: AudioWorkletNodeOptions): AudioWorkletProcessor

  parameterDescriptors?: AudioParamDescriptor[]
}

declare function registerProcessor(
  name: string,
  processorCtor: (new (options?: AudioWorkletNodeOptions) => AudioWorkletProcessor) & {
    parameterDescriptors?: AudioParamDescriptor[]
  }
): void
