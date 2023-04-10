declare namespace wasm_bindgen {
	/* tslint:disable */
	/* eslint-disable */
	/**
	* @param {string} url
	* @param {FarkleSolverWasm} solver
	* @returns {Promise<any>}
	*/
	export function populate_solver(url: string, solver: FarkleSolverWasm): Promise<any>;
	/**
	*/
	export function set_panic_hook(): void;
	/**
	*/
	export enum Action {
	  Stay = 0,
	  Roll = 1,
	}
	/**
	*/
	export class FarkleSolverWasm {
	  free(): void;
	/**
	*/
	  constructor();
	/**
	* @param {number} held_score
	* @param {number} dice_left
	* @param {Int32Array} scores
	* @returns {any}
	*/
	  decide_action_ext(held_score: number, dice_left: number, scores: Int32Array): any;
	/**
	* @param {number} held_score
	* @param {string} roll
	* @param {Int32Array} scores
	* @returns {any}
	*/
	  decide_held_dice_ext(held_score: number, roll: string, scores: Int32Array): any;
	/**
	* @returns {boolean}
	*/
	  get_is_approx(): boolean;
	/**
	* @param {boolean} is_approx
	*/
	  set_is_approx(is_approx: boolean): void;
	}
	
}

declare type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

declare interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly populate_solver: (a: number, b: number, c: number) => number;
  readonly __wbg_farklesolverwasm_free: (a: number) => void;
  readonly farklesolverwasm_new: () => number;
  readonly farklesolverwasm_decide_action_ext: (a: number, b: number, c: number, d: number, e: number) => number;
  readonly farklesolverwasm_decide_held_dice_ext: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly farklesolverwasm_get_is_approx: (a: number) => number;
  readonly farklesolverwasm_set_is_approx: (a: number, b: number) => void;
  readonly set_panic_hook: () => void;
  readonly __wbindgen_export_0: (a: number) => number;
  readonly __wbindgen_export_1: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export_4: (a: number, b: number) => void;
  readonly __wbindgen_export_5: (a: number) => void;
  readonly __wbindgen_export_6: (a: number, b: number, c: number, d: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
declare function wasm_bindgen (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
