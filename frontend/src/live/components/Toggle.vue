<script setup lang="ts">
const props = defineProps({
  label: {
    type: String,
    default: 'Toggle me',
  },
  modelValue: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['update:modelValue'])

const toggleSwitch = () => {
  emit('update:modelValue', !props.modelValue)
}
</script>

<template>
  <label class="inline-flex items-center cursor-pointer">
    <input
      type="checkbox"
      :checked="modelValue"
      @change="toggleSwitch"
      class="sr-only peer"
    />
    <div
      class="relative w-11 h-6 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-800 rounded-full peer bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all border-gray-600 peer-checked:bg-blue-600"
    ></div>
    <span class="ms-3 text-sm font-medium text-gray-300">
      {{ label }}
    </span>
  </label>
</template>
