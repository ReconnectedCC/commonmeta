use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct CommonMeta;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParseResult {
    pub pairs: HashMap<String, String>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParsedPair {
    pub key: String,
    pub value: String,
}

pub fn parse_content(input: &str) -> ParseResult {
    match CommonMeta::parse(Rule::file, input) {
        Ok(mut pairs) => {
            let file = pairs.next().unwrap();
            let mut result_pairs = HashMap::new();

            fn process_pairs(
                pairs: pest::iterators::Pairs<Rule>,
                result_pairs: &mut HashMap<String, String>,
            ) {
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::statement => {
                            process_pairs(pair.into_inner(), result_pairs);
                        }
                        Rule::pair => {
                            process_pairs(pair.into_inner(), result_pairs);
                        }
                        Rule::key_value => {
                            let mut inner_rules = pair.into_inner();
                            let key = inner_rules.next().unwrap().as_str().to_string();
                            let value = inner_rules.next().unwrap().as_str().trim().to_string();
                            result_pairs.insert(key, value);
                        }
                        Rule::key_only => {
                            let mut inner_rules = pair.into_inner();
                            let key = inner_rules.next().unwrap().as_str().to_string();
                            result_pairs.insert(key, "".to_string());
                        }
                        Rule::key => {
                            let key = pair.as_str().to_string();
                            result_pairs.insert(key, "".to_string());
                        }
                        _ => {}
                    }
                }
            }

            process_pairs(file.into_inner(), &mut result_pairs);

            ParseResult {
                pairs: result_pairs,
                success: true,
                error: None,
            }
        }
        Err(e) => ParseResult {
            pairs: HashMap::new(),
            success: false,
            error: Some(format!("{}", e)),
        },
    }
}

pub fn parse_to_json(input: &str) -> String {
    let result = parse_content(input);
    serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
}

pub fn parse(input: &str) -> Result<HashMap<String, String>, String> {
    let result = parse_content(input);
    if result.success {
        Ok(result.pairs)
    } else {
        Err(result.error.unwrap_or("Unknown error".to_string()))
    }
}

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "jni")]
pub mod jni_bindings;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let result = parse("a=b;c=d;").unwrap();
        assert_eq!(result.get("a"), Some(&"b".to_string()));
        assert_eq!(result.get("c"), Some(&"d".to_string()));
    }

    #[test]
    fn test_parse_key_only() {
        let result = parse("flag;enabled=true;disabled;").unwrap();
        assert_eq!(result.get("flag"), Some(&"".to_string()));
        assert_eq!(result.get("enabled"), Some(&"true".to_string()));
        assert_eq!(result.get("disabled"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_email_keys() {
        let result = parse("user@example.com=active;admin.user=inactive;").unwrap();
        assert_eq!(result.get("user@example.com"), Some(&"active".to_string()));
        assert_eq!(result.get("admin.user"), Some(&"inactive".to_string()));
    }

    #[test]
    fn test_parse_multiline() {
        let input = "a=b;\nc=d;\n\nflag;\n";
        let result = parse(input).unwrap();
        assert_eq!(result.get("a"), Some(&"b".to_string()));
        assert_eq!(result.get("c"), Some(&"d".to_string()));
        assert_eq!(result.get("flag"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_json() {
        let json = parse_to_json("a=hello world;b=123;flag;");
        let parsed: ParseResult = serde_json::from_str(&json).unwrap();
        assert!(parsed.success);
        assert_eq!(parsed.pairs.get("a"), Some(&"hello world".to_string()));
        assert_eq!(parsed.pairs.get("flag"), Some(&"".to_string()));
    }

    #[test]
    fn test_parse_semicolon_sequences() {
        let result = parse(";;;a=b;;;c=d;;;").unwrap();
        assert_eq!(result.get("a"), Some(&"b".to_string()));
        assert_eq!(result.get("c"), Some(&"d".to_string()));
    }

    #[test]
    fn test_parse_error() {
        let result = parse(";=1;23=;213;;==");
        assert!(result.is_err());
    }
}
