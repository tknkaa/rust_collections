use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Value {
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

fn tokenize(raw: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let mut buffer = String::new();
    for current in raw.chars() {
        if current == '{' || current == '[' {
            tokens.push(current.to_string());
        } else if current == '}' || current == ']' {
            tokens.push(buffer.clone());
            buffer = current.to_string();
        } else if current == ':' || current == ',' {
            tokens.push(buffer.clone());
            tokens.push(current.to_string());
            buffer.clear();
        } else if current == '\n' || current == ' ' {
            continue;
        } else {
            buffer.push(current);
        }
    }
    let last_char = raw.chars().nth(raw.len() - 1).unwrap();
    tokens.push(last_char.to_string());
    return tokens;
}

fn parse(raw: &str) -> Value {
    let tokens = tokenize(raw);
    let mut cursor = 0;
    return parse_value(&tokens, &mut cursor);
}

fn parse_value(tokens: &Vec<String>, cursor: &mut usize) -> Value {
    let current = tokens.get(*cursor).unwrap();
    if current == "{" {
        return parse_object(&tokens, cursor);
    } else if current == "[" {
        return parse_array(&tokens, cursor);
    } else {
        // primitive
        *cursor += 1;
        return Value::String(current.to_string());
    }
}

fn parse_object(tokens: &Vec<String>, cursor: &mut usize) -> Value {
    let mut obj: HashMap<String, Value> = HashMap::new();
    *cursor += 1;
    loop {
        match tokens.get(*cursor) {
            Some(v) => {
                if v == "}" {
                    break;
                } else {
                    let key = tokens.get(*cursor).unwrap();
                    *cursor += 1;
                    if tokens.get(*cursor).unwrap() == ":" {
                        *cursor += 1;
                    }
                    let value = parse_value(tokens, cursor);
                    obj.insert(key.to_string(), value);
                    *cursor += 1;
                    match tokens.get(*cursor) {
                        Some(v) => {
                            if v == "," {
                                *cursor += 1;
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }
            }
            None => break,
        }
    }
    *cursor += 1;
    return Value::Object(obj);
}

fn parse_array(tokens: &Vec<String>, cursor: &mut usize) -> Value {
    let mut array: Vec<Value> = Vec::new();
    *cursor += 1;
    while tokens.get(*cursor).unwrap() != "]" {
        let value = parse_value(tokens, cursor);
        array.push(value);
        if tokens.get(*cursor).unwrap() == "," {
            *cursor += 1;
        }
    }
    *cursor += 1;
    return Value::Array(array);
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
    fn test_tokenize_with_comma() {
        let input = r#"{"a":1,"b":2}"#;
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec!["{", "\"a\"", ":", "1", ",", "\"b\"", ":", "2", "}"]
        );
    }

    #[test]
    fn test_tokenize_array() {
        let input = "[1,2,3]";
        let tokens = tokenize(input);
        assert_eq!(tokens, vec!["[", "1", ",", "2", ",", "3", "]"]);
    }

    #[test]
    fn test_tokenize_nested() {
        let input = r#"{"arr":[1,2]}"#;
        let tokens = tokenize(input);
        assert_eq!(
            tokens,
            vec!["{", "\"arr\"", ":", "[", "1", ",", "2", "]", "}"]
        );
    }

    #[test]
    fn test_parse_basic() {
        let data = r#"
{
    "name": "John Doe",
    "age": 43,
}
        "#;
        let v: Value = parse(data);

        let mut obj = HashMap::new();
        obj.insert(
            String::from("\"name\""),
            Value::String(String::from("\"John Doe\"")),
        );
        obj.insert(String::from("\"age\""), Value::String(String::from("43")));
        let answer: Value = Value::Object(obj);
        let tokens = tokenize(data);
        println!("{:?}", tokens);
        assert_eq!(v, answer);
    }
}
