use struct_field_derive::StructField;
use json_parser::FieldNames;

#[derive(StructField)]
#[allow(dead_code)]
struct Test {
    field1: usize,
    field2: usize,
}

fn main() {
    println!("{:?}", Test::fields());
    // ["field1", "field2"]
}
