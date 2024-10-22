from pathlib import Path

import librosa
import msgpack
import numpy as np

from sine_wave_speech.lpc import fit_lpc


def main():
    repo_root = Path(__file__).parents[1]
    sr = 8000
    p = 12
    hop_size = 256  # same as DEFAULT_HOP_SIZE but specify in case we change that

    audio, _sr = librosa.load(repo_root / "sentence-original.wav", sr=sr)

    lpc_coefficients, gain, residual = fit_lpc(audio, p=p, hop_size=hop_size)

    # Use msgpack because we're not going to be reading the file anyway (it's a bunch
    # of floats) and it's smaller than JSON.
    output_file = (
        repo_root / "wasm_realtime_sws" / "fixtures" / "fit_lpc_results.msgpack"
    )

    with open(output_file, "wb") as f:
        data = msgpack.packb(
            {
                "audio": audio.astype(np.float32).tolist(),
                "p": p,
                "hop_size": hop_size,
                # Flatten to 1D because it's easier to parse that on the Rust side
                "lpc_coefficients": lpc_coefficients.astype(np.float32)
                .flatten()
                .tolist(),
                "gain": gain.astype(np.float32).tolist(),
                "residual": residual.astype(np.float32).tolist(),
            },
            use_single_float=True,  # use float32 instead of float64
        )
        f.write(data)

    print(f"Saved fit_lpc() args and results to {output_file}")


if __name__ == "__main__":
    main()
