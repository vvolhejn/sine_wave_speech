from pathlib import Path
import librosa
import numpy as np

from sine_wave_speech.lpc import fit_lpc

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
