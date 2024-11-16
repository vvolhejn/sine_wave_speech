<script setup lang="ts">
import {
  getSynthesisParameter,
  SynthesisParameterName as ParameterName,
} from '../synthesisParameters'

const props = defineProps<{
  modelValue: number | string
  label: string
  name: ParameterName
  step?: number | string
}>()

const emit = defineEmits(['update:modelValue', 'change'])

const updateValue = (event: any) => {
  const value = Number(event.target.value)
  emit('update:modelValue', value)
  emit('change', value)
}

const parameter = getSynthesisParameter(props.name)
const id = `slider-${props.name}`
</script>

<template>
  <div>
    <label v-if="label" :for="id" class="block mb-2 text-sm font-medium text-white">
      {{ label }}
    </label>
    <input
      :id="id"
      type="range"
      :value="modelValue"
      @input="updateValue"
      class="w-full h-2 rounded-lg appearance-none cursor-pointer bg-gray-700"
      :min="parameter.minValue"
      :max="parameter.maxValue"
      :step="step || 1"
    />
  </div>
</template>
