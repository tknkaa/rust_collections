# json_parser_rs
```rs
fn main() {
    let data = r#"
    {
        "name": "John Doe",
        "age": 43,
    }
        "#;

    let json = json_parser::parse(data);
    println!("{:?}", json);
    println!("{}", json["name"]);
}
```
```sh
Object({"age": String("43"), "name": String("\"John Doe\"")})
"John Doe"
```
