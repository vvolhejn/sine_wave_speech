<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

import {
  getSynthesisParameter,
  SynthesisParameterName as ParameterName,
} from '../synthesisParameters'

const MAX_X_MOVE_FOR_NO_DRAG = 5

const props = withDefaults(
  defineProps<{
    label: string
    name: ParameterName
    step?: number
  }>(),
  { step: 1 }
)

const model = defineModel<number>({ required: true })

const parameter = getSynthesisParameter(props.name)

const thumbRef = ref<HTMLElement | null>(null)
const trackRef = ref<HTMLElement | null>(null)
const trackRect = computed(() => trackRef.value?.getBoundingClientRect() || null)

type DragInfo = {
  startX: number
  endX: number
  startValue: number
}
const dragInfo = ref<DragInfo | null>(null)

// Calculate percentage for visual positioning
const percentage = computed(() => {
  const range = parameter.maxValue - parameter.minValue
  return ((model.value - parameter.minValue) / range) * 100
})

const valueToX = (value: number) => {
  if (!trackRect.value) {
    throw new Error('trackRect must be defined for valueToX')
  }

  const range = parameter.maxValue - parameter.minValue
  const fraction = (value - parameter.minValue) / range
  const rect = trackRect.value
  const x = fraction * rect.width
  return x
}

const updateValueAbsolute = (clientX: number) => {
  if (!trackRect.value) return

  const rect = trackRect.value
  const position = Math.max(0, Math.min(rect.width, clientX - rect.left))
  const fraction = position / rect.width

  const range = parameter.maxValue - parameter.minValue
  let newValue = parameter.minValue + fraction * range

  if (props.step > 0) {
    newValue = Math.round(newValue / props.step) * props.step
    // Round to 6 digits to prevent floating-point issues
    newValue = Math.round(newValue * 1e6) / 1e6
  }

  newValue = Math.max(parameter.minValue, Math.min(parameter.maxValue, newValue))

  model.value = newValue
}

const updateValueRelative = (clientX: number) => {
  if (!trackRect.value || !dragInfo.value) return

  const rect = trackRect.value
  const deltaX = clientX - dragInfo.value.startX
  const originalX = valueToX(dragInfo.value.startValue)
  updateValueAbsolute(rect.left + originalX + deltaX)
}

const startDragging = (event: MouseEvent | TouchEvent) => {
  if (!trackRef.value) return
  event.preventDefault()

  const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX

  dragInfo.value = {
    startX: clientX,
    endX: clientX,
    startValue: model.value,
  }
}

const onDragging = (event: MouseEvent | TouchEvent) => {
  if (!dragInfo.value) return

  const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX
  updateValueRelative(clientX)
  dragInfo.value.endX = clientX
  event.preventDefault()
}

const stopDragging = (event: MouseEvent | TouchEvent) => {
  if (!dragInfo.value) return

  if (Math.abs(dragInfo.value.endX - dragInfo.value.startX) <= MAX_X_MOVE_FOR_NO_DRAG) {
    // Handle tap-to-seek
    if (trackRef.value) {
      const clientX =
        'touches' in event ? event.changedTouches[0].clientX : event.clientX
      updateValueAbsolute(clientX)
    }
  }

  dragInfo.value = null
}

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
})
</script>

<template>
  <div
    v-if="label"
    class="block mb-2 text-sm font-medium text-white"
    @mousedown="startDragging"
    @touchstart="startDragging"
  >
    {{ label }}
  </div>

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
      :class="{ 'scale-110': dragInfo !== null }"
      :style="{ left: `${percentage}%` }"
    ></div>
  </div>
</template>
