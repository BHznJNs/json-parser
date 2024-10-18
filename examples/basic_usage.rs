fn main() {
    let result = json_parser::parse("true").unwrap();
    println!("{}", result);
}
