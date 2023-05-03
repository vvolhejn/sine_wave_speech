import numpy as np

from sine_wave_speech.lpc import fit_lpc, lpc_coefficients_to_frequencies


def synthesize(
    normalized_frequencies: np.ndarray,
    magnitudes: np.ndarray,
    hop_size: int = 128,
    wave_fn=np.cos,
) -> np.ndarray:
    """Synthesize a signal from sine wave frequencies and magnitudes.

    We don't need to know the sample rate because the frequencies are in radians/s.

    Args:
        normalized_frequencies: Sine wave frequencies in radians/s of shape
            (n_frames, n_channels).
        magnitudes: Sine wave magnitudes of shape (n_frames, n_channels).
        hop_size: Hop size - the number of samples between two consecutive frames.
        wave_fn: The wave function to use. Defaults to np.cos. Can be set to other
            functions like scipy.signal.sawtooth for interesting effects.

    Returns: The synthesized signal.
    """
    assert normalized_frequencies.shape == magnitudes.shape
    assert len(normalized_frequencies.shape) == 2

    n_frames, n_waves = normalized_frequencies.shape
    output_samples = 1 + (n_frames - 1) * hop_size
    output = np.zeros(output_samples)

    for i in range(n_waves):
        cur = synthesize_one(
            normalized_frequencies[:, i], magnitudes[:, i], hop_size, wave_fn
        )
        output += cur

    # Normalize the output signal to [-1, 1] to avoid clipping.
    output /= np.max(np.abs(output))

    return output


def synthesize_one(
    normalized_frequencies: np.ndarray,
    magnitudes: np.ndarray,
    hop_size: int,
    wave_fn=np.cos,
):
    """Synthesize a signal from the frequencies and magnitudes of a single sine wave."""
    assert normalized_frequencies.shape == magnitudes.shape
    assert len(normalized_frequencies.shape) == 1

    normalized_frequencies = upsample(normalized_frequencies, hop_size)
    magnitudes = upsample(magnitudes, hop_size)

    phase = np.cumsum(normalized_frequencies)

    return magnitudes * wave_fn(phase)


def upsample(x: np.ndarray, factor: int) -> np.ndarray:
    """Upsample a signal, stretching it by an integer factor."""
    x = np.nan_to_num(x)  # NaNs can occur at the beginning or end of the signal.
    output_size = (len(x) - 1) * factor + 1
    return np.interp(np.arange(output_size), np.arange(len(x)) * factor, x)


def normalized_to_hz(normalized_frequencies: np.ndarray, sr: int) -> np.ndarray:
    """Convert normalized frequencies (radians/s) to Hz."""
    return normalized_frequencies * sr / (2 * np.pi)


def hz_to_normalized(frequencies_hz: np.ndarray, sr: int) -> np.ndarray:
    """Convert Hz to normalized frequencies (radians/s)."""
    return frequencies_hz * (2 * np.pi) / sr


def to_sine_wave_speech(audio: np.ndarray, n_waves: int = 4) -> np.ndarray:
    """Convert an audio signal to sine wave speech.

    We don't actually need the sample rate of the signal. The sample rate of the
    output signal will be the same as the input signal.

    Args:
        audio (np.ndarray): Audio signal of shape (n_samples,).
        n_waves (int): Number of sine waves to use.
    """
    lpc_coefficients, gain, residual = fit_lpc(audio, p=n_waves * 2)
    frequencies, magnitudes = lpc_coefficients_to_frequencies(lpc_coefficients, gain)

    return synthesize(frequencies, magnitudes)
