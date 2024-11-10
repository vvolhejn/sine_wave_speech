<script setup lang="ts">
const props = defineProps({
  disabled: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['click'])

const handleClick = (event: Event) => {
  if (!props.disabled) {
    emit('click', event)
  }
}
</script>

<template>
  <button
    type="button"
    :disabled="disabled"
    class="hover:bg-blue-500 bg-blue-700 text-white font-bold py-2 px-4 rounded m-2"
    @click="handleClick"
  >
    <slot></slot>
  </button>
</template>

<style scoped>
button:disabled {
  background-color: #a0aec0;
  cursor: not-allowed;
}
</style>
