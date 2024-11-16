<script setup lang="ts">
import { computed } from 'vue'

import { SynthesisParameters } from '../synthesisParameters'
import ParameterSlider from './ParameterSlider.vue'

const parameters = defineModel<SynthesisParameters>({ required: true })

const frequencyQuantizationName = computed(() => {
  const strength = parameters.value.frequencyQuantizationStrength
  const breakpoints = [
    { upTo: 0.5, name: 'Microtonal' },
    { upTo: 1.5, name: 'Chromatic' },
    { upTo: 2.5, name: 'Diatonic' },
    { upTo: 3.0, name: 'Pentatonic' },
  ]
  for (const { upTo, name } of breakpoints) {
    if (strength <= upTo) {
      return name
    }
  }

  throw new Error(`Invalid frequency quantization strength ${strength}`)
})
</script>
<template>
  <div class="grid grid-cols-2 gap-2 w-full">
    <ParameterSlider
      v-model="parameters.hopSizeMultiplier"
      name="hopSizeMultiplier"
      :label="`Step size: ${parameters.hopSizeMultiplier}`"
    />
    <ParameterSlider
      v-model="parameters.nWaves"
      name="nWaves"
      :label="`Number of waves: ${parameters.nWaves}`"
    />
    <ParameterSlider
      v-model="parameters.frequencyQuantizationStrength"
      name="frequencyQuantizationStrength"
      :label="`Scale: ${frequencyQuantizationName}`"
      :step="0.1"
    />
    <ParameterSlider
      v-model="parameters.gainDb"
      name="gainDb"
      :label="`Gain: ${parameters.gainDb} dB`"
    />
    <ParameterSlider
      v-model="parameters.depthOctaves"
      name="depthOctaves"
      :label="`Depth: ${parameters.depthOctaves}`"
      :step="0.1"
    />
  </div>
</template>
