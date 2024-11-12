type SynthesisParameterName =
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
  {
    name: 'depthOctaves',
    defaultValue: 0,
    minValue: 0,
    maxValue: 2,
  },
]

export type SynthesisParameters = {
  [K in SynthesisParameterName]: number
}
