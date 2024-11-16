<script setup lang="ts">
import { computed } from 'vue'

import { SynthesisParameters } from '../synthesisParameters'
import Slider from './Slider.vue'

const synthesisParameters = defineModel<SynthesisParameters>({ required: true })

const frequencyQuantizationName = computed(() => {
  const strength = synthesisParameters.value.frequencyQuantizationStrength
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
  <div class="mt-2">
    <Slider
      v-model="synthesisParameters.hopSizeMultiplier"
      :label="`Step size: ${synthesisParameters.hopSizeMultiplier}`"
      :min="1"
      :max="16"
      id="hop-size-multiplier-slider"
    />
    <Slider
      v-model="synthesisParameters.nWaves"
      :label="`Number of waves: ${synthesisParameters.nWaves}`"
      :min="1"
      :max="16"
      id="n-waves-slider"
    />
    <Slider
      v-model="synthesisParameters.frequencyQuantizationStrength"
      :label="`Scale: ${frequencyQuantizationName}`"
      :min="0"
      :max="3"
      :step="0.1"
      id="frequency-quantization-level-slider"
    />
    <Slider
      v-model="synthesisParameters.gainDb"
      :label="`Gain: ${synthesisParameters.gainDb} dB`"
      :min="-18"
      :max="18"
      id="gain-db-slider"
    />
    <Slider
      v-model="synthesisParameters.depthOctaves"
      :label="`Depth: ${synthesisParameters.depthOctaves}`"
      :min="0"
      :max="2"
      :step="0.1"
      id="dept-octaves-slider"
    />
  </div>
</template>
