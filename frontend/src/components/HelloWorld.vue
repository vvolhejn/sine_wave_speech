<script setup lang="ts">
import { onMounted, ref } from 'vue'
import sentenceSineWave from '../assets/sentence-sine-wave.wav'

defineProps<{ msg: string }>()

const count = ref(0)

const audioContext = new window.AudioContext()

// These get set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)
const playButton = ref<HTMLButtonElement | null>(null)

const isPlaying = ref(false)

onMounted(() => {
  if (!audioElement.value) return

  const track = audioContext.createMediaElementSource(audioElement.value)
  track.connect(audioContext.destination)
})

const onAudioEnded = () => {
  isPlaying.value = false
}

const onClick = () => {
  // Check if context is in suspended state (autoplay policy)
  if (audioContext.state === 'suspended') {
    audioContext.resume()
  }

  if (!audioElement.value) return

  // Play or pause track depending on state
  if (!isPlaying.value) {
    audioElement.value.play()
    isPlaying.value = true
  } else {
    audioElement.value.pause()
    isPlaying.value = false
  }
}
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank"
      >create-vue</a
    >, the official Vue + Vite starter
  </p>
  <p>
    Install
    <a href="https://github.com/vuejs/language-tools" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
  <audio :src="sentenceSineWave" ref="audioElement" @ended="onAudioEnded"></audio>
  <button
    data-playing="false"
    role="switch"
    aria-checked="false"
    @click="onClick"
    ref="playButton"
  >
    <span>Play/Pause</span>
  </button>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
