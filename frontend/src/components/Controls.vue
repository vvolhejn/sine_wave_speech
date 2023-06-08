<script setup lang="ts">
import { onMounted, ref } from 'vue'
import sentence from '../assets/sentence-original.wav'
import swsData from '../assets/virgil-abloh-clip.json'
import { usePlaybackStore } from '../stores/playbackStore'
import { setUpSineWaveSpeechAudio } from '../audio'
// https://colorhunt.co/palette/3e3838ae7c7c6cbbb3efe784
// https://coolors.co/1e152a-4e6766-5ab1bb-a5c882-f7dd72

// const audioContext = new window.AudioContext()
const playbackStore = usePlaybackStore()
playbackStore.setSwsData(swsData)

// This gets set via the ref="..." attribute in the template
const audioElement = ref<HTMLAudioElement | null>(null)

onMounted(() => {
  if (!audioElement.value) return

  const track = playbackStore.audioContext.createMediaElementSource(audioElement.value)
  track.connect(playbackStore.audioContext.destination)

  // Without this little timeout, there is a white screen as the CPU-intensive operation
  // prevents the component from loading - even with async
  setTimeout(setUpSineWaveSpeechAudio, 100)
})

const onAudioEnded = () => {
  playbackStore.setIsPlaying(false)
}

const onClick = () => {
  // Play or pause track depending on state
  if (!playbackStore.isPlaying) {
    playbackStore.playSineWaveSpeech()
    playbackStore.setIsPlaying(true)
  } else {
    playbackStore.audioContext.suspend()
    playbackStore.setIsPlaying(false)
  }
}
</script>

<template>
  <audio :src="sentence" ref="audioElement" @ended="onAudioEnded"></audio>
  <div class="flex flex-col min-h-screen justify-center">
    <button @click="onClick">
      <h1 class="text-8xl text-center mix-blend-difference font-[Playfair] italic">
        Sine Wave Speech
      </h1>
    </button>
  </div>
</template>
