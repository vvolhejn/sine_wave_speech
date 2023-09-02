# Sine wave speech

**Check out https://www.sinewavespeech.com/ to try out sine wave speech in an interactive way.**
The code for the website lives under `frontend/`.

Sine wave speech is a method of converting speech to sounds that are utterly unintelligible on the first listen,
but with a little practice – hearing a few before-and-after examples – become easy to follow.

This repo also contains a Python package for converting audio recordings to sine wave speech.
For an example of how to use the package, see [sine_wave_speech_example.ipynb](sine_wave_speech_example.ipynb).

## What is sine wave speech?

In short, using fancy signal processing methods ([LPC](https://ccrma.stanford.edu/~hskim08/lpc/)),
we determine the four most important frequencies at every point in time and _only keep those_.

You can see on the spectrogram of the modified audio that only four frequencies are ever active at a time:

![](sentence-sine-wave.png)

Listen to [sentence-sine-wave.wav](sentence-sine-wave.wav).

Check out [this page](https://vvolhejn.github.io/2023/08/20/sinewavespeech-com.html) for a detailed explanation.

