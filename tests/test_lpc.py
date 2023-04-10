from pathlib import Path
import librosa
import numpy as np

from sine_wave_speech.lpc import fit_lpc, lpc_coefficients_to_frequencies

SCRIPT_DIR = Path(__file__).parent


def test_fit_lpc():
    audio, _ = librosa.load(SCRIPT_DIR / "sentence.wav", sr=8000)

    assert audio.shape == (12245,)

    lpc_coefficients, gain, residual = fit_lpc(audio, p=8)

    assert lpc_coefficients.shape == (95, 9)
    assert gain.shape == (95,)
    assert residual.shape == (12224,)

    assert lpc_coefficients[0, 0] == 1
    assert np.isclose(lpc_coefficients.sum(), 237.60091124501957)


def test_lpc_coefficients_to_frequencies():
    audio, _ = librosa.load(SCRIPT_DIR / "sentence.wav", sr=8000)

    lpc_coefficients, gain, _ = fit_lpc(audio, p=8)

    frequencies, magnitudes = lpc_coefficients_to_frequencies(lpc_coefficients, gain)

    assert frequencies.shape == (95, 4)
    assert magnitudes.shape == (95, 4)

    assert np.isclose(frequencies.sum(), 616.962528185217)
    assert np.isclose(magnitudes.sum(), 31.225811085715158)
