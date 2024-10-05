use struct_field_derive::ListedFieldNames;
use json_parser::ListedFieldNames;

#[derive(ListedFieldNames)]
#[allow(dead_code)]
struct Test {
    field1: usize,
    field2: usize,
}

fn main() {
    println!("{:?}", Test::fields());
    // ["field1", "field2"]
}
