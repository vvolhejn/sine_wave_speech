from pathlib import Path

import librosa
import msgpack
import numpy as np

from sine_wave_speech.lpc import fit_lpc, lpc_coefficients_to_frequencies


def main():
    repo_root = Path(__file__).parents[1]
    sr = 8000
    n_waves = 4
    hop_size = 256  # same as DEFAULT_HOP_SIZE but specify in case we change that

    audio, _sr = librosa.load(repo_root / "sentence-original.wav", sr=sr)

    lpc_coefficients, gain, residual = fit_lpc(audio, p=n_waves * 2, hop_size=hop_size)

    frequencies, magnitudes = lpc_coefficients_to_frequencies(lpc_coefficients, gain)

    # Use msgpack because we're not going to be reading the file anyway (it's a bunch
    # of floats) and it's smaller than JSON.
    output_file = (
        repo_root / "wasm_realtime_sws" / "fixtures" / "python_sws_results.msgpack"
    )

    with open(output_file, "wb") as f:
        data = msgpack.packb(
            {
                "audio": audio.tolist(),
                "n_waves": n_waves,
                "hop_size": hop_size,
                # Flatten to 1D because it's easier to parse that on the Rust side
                "lpc_coefficients": lpc_coefficients.flatten().tolist(),
                "gain": gain.tolist(),
                "residual": residual.tolist(),
                "frequencies": frequencies.flatten().tolist(),
                "magnitudes": magnitudes.flatten().tolist(),
            },
            use_single_float=True,  # use float32 instead of float64
        )
        f.write(data)

    print(f"Saved fit_lpc() args and results to {output_file}")


if __name__ == "__main__":
    main()
