export const getAudioBuffer = async (audioContext: AudioContext, audioFile: string) => {
  const response = await fetch(audioFile)
  const arrayBuffer = await response.arrayBuffer()
  const audioBuffer = await audioContext.decodeAudioData(arrayBuffer)
  return audioBuffer
}

export const getWebAudioMediaStream = async () => {
  if (!window.navigator.mediaDevices) {
    throw new Error('This browser does not support web audio or it is not enabled.')
  }

  try {
    const result = await window.navigator.mediaDevices.getUserMedia({
      audio: true,
      video: false,
    })

    return result
  } catch (e: any) {
    switch (e.name) {
      case 'NotAllowedError':
        throw new Error(
          'A recording device was found but has been disallowed for this application. Enable the device in the browser settings.'
        )

      case 'NotFoundError':
        throw new Error(
          'No recording device was found. Please attach a microphone and click Retry.'
        )

      default:
        throw e
    }
  }
}

/**
 * Calculates decibels relative to full scale (dBFS) from a linear magnitude value
 * @param magnitude - Linear magnitude (amplitude) value between 0 and 1
 * @returns dBFS value (always <= 0). Will be -Infinity if magnitude is 0.
 * @throws Error if magnitude is negative or greater than 1
 */
export const magnitudeToDbfs = (magnitude: number): number => {
  // Validate input
  if (magnitude < 0 || magnitude > 1) {
    throw new Error('magnitude must be between 0 and 1')
  }

  // Calculate dBFS: 20 * log10(magnitude).
  return 20 * Math.log10(magnitude)
}

const trimAudioBuffer = (
  audioContext: AudioContext,
  audioBuffer: AudioBuffer,
  nSamples: number
): AudioBuffer => {
  const newBuffer = audioContext.createBuffer(
    audioBuffer.numberOfChannels,
    nSamples,
    audioBuffer.sampleRate
  )
  for (let i = 0; i < audioBuffer.numberOfChannels; i++) {
    newBuffer.copyToChannel(audioBuffer.getChannelData(i).slice(0, nSamples), i)
  }
  return newBuffer
}

export const trimAudioBufferToMultipleOf = (
  audioContext: AudioContext,
  audioBuffer: AudioBuffer,
  multiple: number
): AudioBuffer => {
  const nSamples = Math.floor(audioBuffer.length / multiple) * multiple
  return trimAudioBuffer(audioContext, audioBuffer, nSamples)
}
