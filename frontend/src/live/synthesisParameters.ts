export type SynthesisParameterName =
  | 'frequencyQuantizationStrength'
  | 'hopSizeMultiplier'
  | 'nWaves'
  | 'gainDb'
  | 'depthOctaves'

export type SynthesisParameter = {
  name: SynthesisParameterName
  defaultValue: number
  minValue: number
  maxValue: number
}

export const synthesisParameters: SynthesisParameter[] = [
  {
    name: 'frequencyQuantizationStrength',
    defaultValue: 2.0,
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
  {
    name: 'depthOctaves',
    defaultValue: 0,
    minValue: 0,
    maxValue: 2,
  },
]

export const getSynthesisParameter = (
  name: SynthesisParameterName
): SynthesisParameter => {
  const parameter = synthesisParameters.find((p) => p.name === name)
  if (!parameter) {
    throw new Error(`Synthesis parameter not found: ${name}`)
  }
  return parameter
}

export const getDefaultSynthesisParameters = (): SynthesisParameters => {
  const result = {} as SynthesisParameters
  synthesisParameters.forEach((parameter) => {
    result[parameter.name] = parameter.defaultValue
  })
  return result
}

export type SynthesisParameters = {
  [K in SynthesisParameterName]: number
}
