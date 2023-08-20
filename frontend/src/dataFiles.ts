/**
 * Centralizes the imports of data files for easy switching between clips.
 * I'm sure there's more elegant ways to do this than commenting/uncommenting lines,
 * but this works for now.
 */
// import explanationSwsData from './assets/explanation-1-short.json'
// import explanationOriginalAudio from './assets/explanation-1-short.mp3'
// import explanationSubtitles from './assets/explanation-1-short.srt?raw'
import explanationSwsData from './assets/explanation-1.json'
import explanationOriginalAudio from './assets/explanation-1.mp3'
import explanationSubtitles from './assets/explanation-1.srt?raw'

export const originalAudio = explanationOriginalAudio
export const swsData = explanationSwsData
export const subtitles = explanationSubtitles
