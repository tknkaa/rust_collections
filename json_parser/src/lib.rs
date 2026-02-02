use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Value {
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

fn tokenize(raw: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer = String::new();
    for value in raw.chars() {
        if value == '{' || value == '[' {
            tokens.push(value.to_string());
        } else if value == '}' || value == ']' || value == ':' {
            if buffer.len() > 0 {
                tokens.push(buffer.clone());
            }
            buffer.clear();
            tokens.push(value.to_string());
        } else {
            buffer.push(value);
        }
    }
    return tokens;
}

fn parse(raw: &str) -> Value {
    return Value::Bool(false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_object() {
        let input = r#"{"name":"John"}"#;
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["{", "\"name\"", ":", "\"John\"", "}"]);
    }

    #[test]
    fn test_tokenize_array() {
        let input = "[1,2,3]";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["[", "1,2,3", "]"]);
    }

    #[test]
    fn test_tokenize_nested() {
        let input = r#"{"arr":[1,2]}"#;
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["{", "\"arr\"", ":", "[", "1,2", "]", "}"]);
    }

    #[test]
    fn basic_case() {
        let data = r#"
{
    "name": "John Doe",
    "age": 43,
}
        "#;
        let v: Value = parse(data);

        let mut obj = HashMap::new();
        obj.insert("name".to_string(), Value::String("John Doe".to_string()));
        obj.insert("age".to_string(), Value::Number(43));
        let answer: Value = Value::Object(obj);
        assert_eq!(v, answer);
    }
}
