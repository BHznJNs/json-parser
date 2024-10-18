use json_parser::ListedFieldNames;

#[derive(ListedFieldNames)]
#[allow(dead_code)]
struct TestStruct {
    field1: usize,
    field2: usize,
}

fn main() {
    println!("{:?}", TestStruct::fields());
    // ["field1", "field2"]
}
