export interface ParseResult {
  pairs: Record<string, string>;
  success: boolean;
  error?: string;
}

export declare class CommonMeta {
  static initialize(): Promise<void>;
  static parse(input: string): Promise<ParseResult>;
  static parsePairs(input: string): Promise<Record<string, string> | null>;
  static parseToJson(input: string): Promise<string>;
  static validate(input: string): Promise<boolean>;
  static getError(input: string): Promise<string | null>;
}

export default CommonMeta;
