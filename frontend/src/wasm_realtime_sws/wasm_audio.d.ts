/* tslint:disable */
/* eslint-disable */
/**
*/
export enum FrequencyQuantizationType {
  Chromatic = 0,
  Diatonic = 1,
  Pentatonic = 2,
}
/**
* Note that the converter doesn't care about the sample rate,
*/
export class SineWaveSpeechConverter {
  free(): void;
/**
* @param {number} n_waves
* @param {number} hop_size
* @param {number} sample_rate
* @returns {SineWaveSpeechConverter}
*/
  static new(n_waves: number, hop_size: number, sample_rate: number): SineWaveSpeechConverter;
/**
* @param {Float32Array} audio_samples
* @returns {Float32Array}
*/
  get_frequencies_and_magnitudes(audio_samples: Float32Array): Float32Array;
/**
* @param {Float32Array} frequencies
* @param {FrequencyQuantizationType | undefined} [quantization_type]
* @returns {Float32Array}
*/
  quantize_frequencies(frequencies: Float32Array, quantization_type?: FrequencyQuantizationType): Float32Array;
/**
* @param {Float32Array} frequencies
* @param {number} quantization_strength
* @returns {Float32Array}
*/
  quantize_frequencies_continuous(frequencies: Float32Array, quantization_strength: number): Float32Array;
/**
* Add depth by lowering the first frequencies more than the last ones.
* Specifically, the first frequency is lowered by `width` octaves, and the last frequency
* is left unchanged. The deepening of the other frequencies is a linear interpolation.
* @param {Float32Array} frequencies
* @param {number} width
* @returns {Float32Array}
*/
  add_depth(frequencies: Float32Array, width: number): Float32Array;
/**
* @param {Float32Array} frequencies
* @param {Float32Array} magnitudes
* @param {Float32Array} first_phases
* @returns {Float32Array}
*/
  synthesize(frequencies: Float32Array, magnitudes: Float32Array, first_phases: Float32Array): Float32Array;
/**
*/
  hop_size: number;
/**
*/
  n_waves: number;
/**
*/
  sample_rate: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_sinewavespeechconverter_free: (a: number) => void;
  readonly __wbg_get_sinewavespeechconverter_n_waves: (a: number) => number;
  readonly __wbg_set_sinewavespeechconverter_n_waves: (a: number, b: number) => void;
  readonly __wbg_get_sinewavespeechconverter_hop_size: (a: number) => number;
  readonly __wbg_set_sinewavespeechconverter_hop_size: (a: number, b: number) => void;
  readonly __wbg_get_sinewavespeechconverter_sample_rate: (a: number) => number;
  readonly __wbg_set_sinewavespeechconverter_sample_rate: (a: number, b: number) => void;
  readonly sinewavespeechconverter_new: (a: number, b: number, c: number) => number;
  readonly sinewavespeechconverter_get_frequencies_and_magnitudes: (a: number, b: number, c: number, d: number) => void;
  readonly sinewavespeechconverter_quantize_frequencies: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly sinewavespeechconverter_quantize_frequencies_continuous: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly sinewavespeechconverter_add_depth: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly sinewavespeechconverter_synthesize: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
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
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
