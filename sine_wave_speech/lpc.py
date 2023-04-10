import numpy as np
import scipy.signal


def fit_lpc(audio: np.ndarray, p=12, hop_size=128, window_size=None):
    """Compute the LPC coefficients and gain for each frame of audio.

    Args:
        audio (np.ndarray): Audio signal.
        p (int, optional): Order of the LPC filter. Defaults to 12.
        hop_size (int, optional): Hop size. Defaults to 128.
        window_size (int, optional): Window size. Defaults to 2 * hop_size.
    """
    if window_size is None:
        window_size = 2 * hop_size

    n_hops = len(audio) // hop_size

    # Zero-pad the audio to make sure we have enough samples for the last window
    audio = np.concatenate(
        [
            np.zeros((window_size - hop_size) // 2),
            audio,
            np.zeros((window_size - hop_size) // 2),
        ]
    )

    lpc_coefficients = np.zeros((n_hops, p + 1))
    gain = np.zeros(n_hops)
    residual = np.zeros((n_hops - 1) * hop_size + window_size)

    # Apply a pre-emphasis (high-pass) filter
    audio = scipy.signal.lfilter(np.array([1.0, -0.9]), 1, audio)

    for hop in range(n_hops):
        cur_audio = audio[hop * hop_size : hop * hop_size + window_size]
        windowed_audio = cur_audio * scipy.signal.get_window("hann", window_size)

        autocorrelated = scipy.signal.correlate(windowed_audio, windowed_audio)
        autocorrelated = autocorrelated[window_size - 1 : window_size + p]

        try:
            cur_lpc_coefficients = scipy.linalg.solve_toeplitz(
                autocorrelated[:p], autocorrelated[1 : p + 1]
            )
        except scipy.linalg.LinAlgError:  # "Singular principal minor"
            continue

        cur_lpc_coefficients = np.concatenate([[1], -cur_lpc_coefficients])
        cur_residual = scipy.signal.lfilter(cur_lpc_coefficients, 1, windowed_audio)
        cur_gain = np.sqrt(np.mean(cur_residual**2))

        lpc_coefficients[hop] = cur_lpc_coefficients
        gain[hop] = cur_gain
        residual[hop * hop_size : hop * hop_size + window_size] += (
            cur_residual / cur_gain
        )

    residual = residual[(window_size - hop_size) // 2 :]

    return lpc_coefficients, gain, residual
