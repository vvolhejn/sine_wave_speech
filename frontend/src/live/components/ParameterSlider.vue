<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

import {
  getSynthesisParameter,
  SynthesisParameterName as ParameterName,
} from '../synthesisParameters'

const props = defineProps<{
  label: string
  name: ParameterName
  step?: number | string
}>()

const model = defineModel<number>()
const emit = defineEmits(['change'])

const parameter = getSynthesisParameter(props.name)
const id = `slider-${props.name}`

const thumbRef = ref<HTMLElement | null>(null)
const trackRef = ref<HTMLElement | null>(null)
const isDragging = ref(false)
const trackRect = ref<DOMRect | null>(null)
const dragStartX = ref<number>(0)
const dragStartValue = ref<number>(0)

// Convert string values to numbers and handle validation
const currentValue = computed(() => Number(model.value))
const minValue = computed(() => Number(parameter.minValue))
const maxValue = computed(() => Number(parameter.maxValue))
const stepValue = computed(() => Number(props.step || 1))

// Calculate percentage for visual positioning
const percentage = computed(() => {
  const range = maxValue.value - minValue.value
  return ((currentValue.value - minValue.value) / range) * 100
})

const updateValueAbsolute = (clientX: number) => {
  if (!trackRect.value) return

  const rect = trackRect.value
  const position = Math.max(0, Math.min(rect.width, clientX - rect.left))
  const percentage = position / rect.width

  const range = maxValue.value - minValue.value
  let newValue = minValue.value + percentage * range

  if (stepValue.value > 0) {
    newValue = Math.round(newValue / stepValue.value) * stepValue.value
  }

  newValue = Math.max(minValue.value, Math.min(maxValue.value, newValue))

  if (newValue !== currentValue.value) {
    model.value = newValue
    emit('change', newValue)
  }
}

const updateValueRelative = (clientX: number) => {
  if (!trackRect.value) return

  const rect = trackRect.value
  const deltaX = clientX - dragStartX.value
  const deltaPercentage = deltaX / rect.width
  const range = maxValue.value - minValue.value
  let newValue = dragStartValue.value + deltaPercentage * range

  if (stepValue.value > 0) {
    newValue = Math.round(newValue / stepValue.value) * stepValue.value
    // Round to avoid floating point errors
    newValue = Math.round(newValue * 1e6) / 1e6
  }

  newValue = Math.max(minValue.value, Math.min(maxValue.value, newValue))

  if (newValue !== currentValue.value) {
    model.value = newValue
    emit('change', newValue)
  }
}

let clickTimer: number | null = null
const startDragging = (event: MouseEvent | TouchEvent) => {
  if (!trackRef.value) return
  event.preventDefault()

  const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX

  // Store initial positions for relative dragging
  dragStartX.value = clientX
  dragStartValue.value = currentValue.value

  // Use setTimeout to determine if this was a tap or drag
  clickTimer = window.setTimeout(() => {
    isDragging.value = true
    trackRect.value = trackRef.value?.getBoundingClientRect() || null
    clickTimer = null
  }, 100) // Short delay to distinguish between tap and drag
}

const onDragging = (event: MouseEvent | TouchEvent) => {
  if (!isDragging.value) return

  const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX
  updateValueRelative(clientX)
  event.preventDefault()
}

const stopDragging = (event: MouseEvent | TouchEvent) => {
  // If clickTimer exists, this was a tap (not a drag)
  if (clickTimer !== null) {
    clearTimeout(clickTimer)
    clickTimer = null

    // Handle tap-to-seek
    if (trackRef.value) {
      trackRect.value = trackRef.value.getBoundingClientRect()
      const clientX =
        'touches' in event ? event.changedTouches[0].clientX : event.clientX
      updateValueAbsolute(clientX)
    }
  }

  isDragging.value = false
  trackRect.value = null
}

// Event listeners
onMounted(() => {
  document.addEventListener('mousemove', onDragging)
  document.addEventListener('mouseup', stopDragging)
  document.addEventListener('touchmove', onDragging, { passive: false })
  document.addEventListener('touchend', stopDragging)
})

onUnmounted(() => {
  document.removeEventListener('mousemove', onDragging)
  document.removeEventListener('mouseup', stopDragging)
  document.removeEventListener('touchmove', onDragging)
  document.removeEventListener('touchend', stopDragging)
  if (clickTimer !== null) {
    clearTimeout(clickTimer)
  }
})
</script>

<template>
  <label v-if="label" :for="id" class="block mb-2 text-sm font-medium text-white">
    {{ label }}
  </label>

  <div
    ref="trackRef"
    class="relative w-full h-2 rounded-lg bg-gray-700 cursor-pointer touch-none"
    @mousedown="startDragging"
    @touchstart="startDragging"
  >
    <!-- Active track -->
    <div
      class="absolute h-full bg-blue-500 rounded-lg transition-all duration-75"
      :style="{ width: `${percentage}%` }"
    ></div>

    <!-- Thumb -->
    <div
      ref="thumbRef"
      class="absolute top-1/2 -translate-y-1/2 w-4 h-4 -ml-2 bg-white rounded-full shadow-md border border-gray-300 transition-transform duration-75"
      :class="{ 'scale-110': isDragging }"
      :style="{ left: `${percentage}%` }"
    ></div>

    <!-- Value tooltip -->
    <div
      class="absolute -top-8 px-2 py-1 bg-gray-900 text-white text-xs rounded transform -translate-x-1/2 pointer-events-none"
      :class="{ 'opacity-100': isDragging, 'opacity-0': !isDragging }"
      :style="{ left: `${percentage}%` }"
    >
      {{ currentValue.toFixed(1) }}
    </div>
  </div>
</template>
