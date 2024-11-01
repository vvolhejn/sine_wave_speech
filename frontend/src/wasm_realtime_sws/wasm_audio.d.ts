/* tslint:disable */
/* eslint-disable */
/**
 * Note that the converter doesn't care about the sample rate,
 */
export class SineWaveSpeechConverter {
  free(): void;
  /**
   * @param {number} n_waves
   * @param {number} hop_size
   * @returns {SineWaveSpeechConverter}
   */
  static new(n_waves: number, hop_size: number): SineWaveSpeechConverter;
  /**
   * @param {Float32Array} audio_samples
   * @returns {Float32Array}
   */
  convert(audio_samples: Float32Array): Float32Array;
  hop_size: number;
  n_waves: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_sinewavespeechconverter_free: (a: number, b: number) => void;
  readonly __wbg_get_sinewavespeechconverter_n_waves: (a: number) => number;
  readonly __wbg_set_sinewavespeechconverter_n_waves: (a: number, b: number) => void;
  readonly __wbg_get_sinewavespeechconverter_hop_size: (a: number) => number;
  readonly __wbg_set_sinewavespeechconverter_hop_size: (a: number, b: number) => void;
  readonly sinewavespeechconverter_new: (a: number, b: number) => number;
  readonly sinewavespeechconverter_convert: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
