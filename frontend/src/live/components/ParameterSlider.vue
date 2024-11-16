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
const hitboxRef = ref<HTMLElement | null>(null)
// Here we take use the rectangle of the hitbox but since we only care about the width,
// this is the same as the actual track. The elements differ only in height.
const trackRect = computed(() => hitboxRef.value?.getBoundingClientRect() || null)

type DragInfo = {
  startX: number
  endX: number
  startValue: number
  allowAbsolute: boolean
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

const startDragging = (event: MouseEvent | TouchEvent, allowAbsolute: boolean) => {
  if (!hitboxRef.value) return
  event.preventDefault()

  const clientX = 'touches' in event ? event.touches[0].clientX : event.clientX

  dragInfo.value = {
    startX: clientX,
    endX: clientX,
    startValue: model.value,
    allowAbsolute: allowAbsolute,
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

  if (
    // Absolute movement is not allowed when we drag the label, not the track or thumb
    dragInfo.value.allowAbsolute &&
    Math.abs(dragInfo.value.endX - dragInfo.value.startX) <= MAX_X_MOVE_FOR_NO_DRAG
  ) {
    if (hitboxRef.value) {
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
    @mousedown="(e) => startDragging(e, false)"
    @touchstart="(e) => startDragging(e, false)"
  >
    {{ label }}
  </div>

  <!-- An outer hitbox to make dragging easier, especially on desktop.
  This way you don't have to exactly hit the track. -->
  <div
    ref="hitboxRef"
    class="w-full h-7 rounded-lg flex cursor-pointer touch-none"
    @mousedown="(e) => startDragging(e, true)"
    @touchstart="(e) => startDragging(e, true)"
  >
    <div
      class="relative w-full h-2 rounded-lg bg-gray-700 m-auto"
      @mousedown="(e) => startDragging(e, true)"
      @touchstart="(e) => startDragging(e, true)"
    >
      <!-- Active track -->
      <div
        class="absolute h-full bg-blue-500 rounded-lg"
        :style="{ width: `${percentage}%` }"
      ></div>

      <!-- Thumb -->
      <div
        ref="thumbRef"
        class="absolute top-1/2 -translate-y-1/2 w-4 h-4 -ml-2 bg-white rounded-full shadow-md border border-gray-300 transition-transform duration-150"
        :class="{ 'scale-110': dragInfo !== null }"
        :style="{ left: `${percentage}%` }"
      ></div>
    </div>
  </div>
</template>
