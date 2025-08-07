import init, {
  parse_to_json,
  parse_pairs,
  validate,
  get_error
} from './pkg/commonmeta.js?init';

import wasmUrl from './pkg/commonmeta_bg.wasm?url'; // static URL to binary asset

export interface ParseResult {
  pairs: Record<string, string>;
  success: boolean;
  error?: string;
}

export class CommonMeta {
  private static initialized = false;
  private static initializingPromise: Promise<void> | null = null;

  static async initialize() {
    if (this.initialized) {
      return;
    }
    if (this.initializingPromise) {
      // Warn if initialization is already in progress
      console.warn('CommonMeta WASM initialization already in progress. Awaiting existing initialization.');
      await this.initializingPromise;
      return;
    }
    this.initializingPromise = (async () => {
      if (import.meta.env && import.meta.env.MODE) { // TODO: this is most likely Vite
        await init(wasmUrl);
      } else {
        await init();
      }
      this.initialized = true;
      this.initializingPromise = null;
    })();
    await this.initializingPromise;
  }

  // Async APIs
  static async parse(input: string): Promise<ParseResult> {
    await this.initialize();
    const jsonResult = parse_to_json(input);
    return JSON.parse(jsonResult);
  }

  static async parsePairs(input: string): Promise<Record<string, string> | null> {
    await this.initialize();
    const result = parse_pairs(input);
    return result || null;
  }

  static async parseToJson(input: string): Promise<string> {
    await this.initialize();
    return parse_to_json(input);
  }

  static async validate(input: string): Promise<boolean> {
    await this.initialize();
    return validate(input);
  }

  static async getError(input: string): Promise<string | null> {
    await this.initialize();
    return get_error(input) || null;
  }

  // Synchronous APIs (require that WASM is already initialized)
  static parseSync(input: string): ParseResult {
    if (!this.initialized) {
      throw new Error('CommonMeta WASM not initialized. Call CommonMeta.initialize() first.');
    }
    const jsonResult = parse_to_json(input);
    return JSON.parse(jsonResult);
  }

  static parsePairsSync(input: string): Record<string, string> | null {
    if (!this.initialized) {
      throw new Error('CommonMeta WASM not initialized. Call CommonMeta.initialize() first.');
    }
    const result = parse_pairs(input);
    return result || null;
  }

  static parseToJsonSync(input: string): string {
    if (!this.initialized) {
      throw new Error('CommonMeta WASM not initialized. Call CommonMeta.initialize() first.');
    }
    return parse_to_json(input);
  }

  static validateSync(input: string): boolean {
    if (!this.initialized) {
      throw new Error('CommonMeta WASM not initialized. Call CommonMeta.initialize() first.');
    }
    return validate(input);
  }

  static getErrorSync(input: string): string | null {
    if (!this.initialized) {
      throw new Error('CommonMeta WASM not initialized. Call CommonMeta.initialize() first.');
    }
    return get_error(input) || null;
  }
}

export default CommonMeta;
