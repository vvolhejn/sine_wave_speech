import numpy as np
from sine_wave_speech.lpc import DEFAULT_HOP_SIZE

from sine_wave_speech.synthesis import normalized_to_hz


def to_json(
    frequencies: np.ndarray,
    magnitudes: np.ndarray,
    sr: int,
    hop_size: int = DEFAULT_HOP_SIZE,
):
    return {
        "sr": sr,
        "hopSize": hop_size,
        "frequencies": normalized_to_hz(frequencies, sr=sr).tolist(),
        "magnitudes": magnitudes.tolist(),
    }
