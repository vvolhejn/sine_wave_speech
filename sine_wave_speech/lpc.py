import numpy as np
import scipy.signal

DEFAULT_HOP_SIZE = 256


def fit_lpc(audio: np.ndarray, p=12, hop_size=DEFAULT_HOP_SIZE, window_size=None):
    """Compute Linear Predictive Coding coefficients and gain for each frame of audio.

    Args:
        audio (np.ndarray): Audio signal.
        p (int, optional): Order of the LPC filter. Defaults to 12.
        hop_size (int, optional): Hop size. Defaults to 256.
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
            print("Singular principal minor")
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


def lpc_coefficients_to_frequencies(lpc_coefficients, gain):
    """Convert LPC coefficients and gain to normalized_frequencies and magnitudes.

    Args:
        lpc_coefficients (np.ndarray): LPC coefficients of shape (n_hops, p + 1).
        gain (np.ndarray): Gain of shape (n_hops,).

    Returns:
        frequencies (np.ndarray): Frequencies of shape (n_hops, p // 2). The unit
            is radians/sample.
        magnitudes (np.ndarray): Magnitudes of shape (n_hops, p // 2).
    """
    n_hops = lpc_coefficients.shape[0]
    p = lpc_coefficients.shape[1] - 1

    frequencies = np.zeros((n_hops, p // 2))
    magnitudes = np.zeros((n_hops, p // 2))

    for hop in range(n_hops):
        cur_lpc_coefficients = lpc_coefficients[hop, 1 : p + 1]

        # The roots of the LPC polynomial are the frequencies of the sinusoids
        # that are present in the signal.
        roots = np.roots(np.concatenate([[1], cur_lpc_coefficients]))

        cur_frequencies = np.angle(roots)

        # frequencies of 0 and pi correspond to real-only roots and seem to be useless.
        # (I don't understand the math behind this, but it seems to work).
        # The pi frequency in radians/sample corresponds to sr/2 Hz.
        # Setting it to -pi instead of pi makes us filter the root out later.
        cur_frequencies[cur_frequencies == np.pi] = -np.pi

        cur_magnitudes = gain[hop] / (1 - np.abs(roots))

        # Sort the frequencies so that the sine waves don't cross when we upsample.
        ix = np.argsort(cur_frequencies)

        # Each frequency is repeated twice, once with a positive angle and once
        # with a negative angle. We only want to keep the positive angle.
        ix = ix[cur_frequencies[ix] > 0]

        frequencies[hop, : len(ix)] = cur_frequencies[ix][: frequencies.shape[1]]
        magnitudes[hop, : len(ix)] = cur_magnitudes[ix][: frequencies.shape[1]]

    return frequencies, magnitudes
