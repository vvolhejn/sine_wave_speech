import numpy as np


def synthesize(
    frequencies: np.ndarray,
    magnitudes: np.ndarray,
    sr: int,
    hop_size=128,
    wave_fn=np.cos,
):
    assert frequencies.shape == magnitudes.shape
    assert len(frequencies.shape) == 2

    frames, channels = frequencies.shape

    output_samples = 1 + (frames - 1) * hop_size

    output = np.zeros(output_samples)

    for i in range(channels):
        cur = synthesize_one(
            frequencies[:, i], magnitudes[:, i], sr, hop_size, wave_fn
        )
        output += cur

    return output


def synthesize_one(
    frequencies: np.ndarray, magnitudes: np.ndarray, sr, hop_size, wave_fn=np.cos
):
    assert frequencies.shape == magnitudes.shape
    assert len(frequencies.shape) == 1

    def process(x):
        x = np.nan_to_num(x)
        x = upsample(x, hop_size)
        return x

    frequencies = process(frequencies)
    magnitudes = process(magnitudes)

    frequencies_hz = 2 * np.pi / sr * frequencies
    phase = np.cumsum(frequencies_hz)

    return magnitudes * wave_fn(phase)


def upsample(x, factor):
    output_size = (len(x) - 1) * factor + 1
    return np.interp(np.arange(output_size), np.arange(len(x)) * factor, x)
