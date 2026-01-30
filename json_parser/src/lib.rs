use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Value {
    Bool(bool),
    Number(i64),
    String(String),
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}

fn parse(raw: &str) -> Value {
    return Value::Bool(false);
}

#[cfg(test)]
mod tests {
    use super::*;

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
