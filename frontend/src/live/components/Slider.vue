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
      :min="min"
      :max="max"
      :step="step"
    />
  </div>
</template>

<script setup>
import { defineProps, defineEmits } from 'vue'

const props = defineProps({
  modelValue: {
    type: [Number, String],
    default: 50,
  },
  label: {
    type: String,
    default: '',
  },
  id: {
    type: String,
    default: 'default-range',
  },
  min: {
    type: [Number, String],
    default: 0,
  },
  max: {
    type: [Number, String],
    default: 100,
  },
  step: {
    type: [Number, String],
    default: 1,
  },
})

const emit = defineEmits(['update:modelValue', 'change'])

const updateValue = (event) => {
  const value = Number(event.target.value)
  emit('update:modelValue', value)
  emit('change', value)
}
</script>
