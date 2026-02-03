fn main() {
     let data = r#"
    {
        "name": "John Doe",
        "age": 43,
    }
        "#;
    
    let json = json_parser::parse(data);
    println!("{:?}", json);
    println!("{:?}", json["\"name\""]);
}

