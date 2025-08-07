import init, {
  parse_to_json,
  parse_to_object,
  parse_pairs,
  validate,
  get_error
} from './pkg/commonmeta.js';

export interface ParseResult {
  pairs: Record<string, string>;
  success: boolean;
  error?: string;
}

export class CommonMeta {
  private static initialized = false;

  static async initialize() {
    if (!this.initialized) {
      await init();
      this.initialized = true;
    }
  }

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
}

export default CommonMeta;
