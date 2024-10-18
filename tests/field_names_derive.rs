use json_parser::ListedFieldNames;

#[derive(ListedFieldNames)]
#[allow(dead_code)]
struct TestStruct {
    field1: usize,
    field2: usize,
}

#[test]
#[allow(non_snake_case)]
fn test_derive_proc_macro_ListedFieldNames() {
    assert_eq!(TestStruct::fields(), vec!["field1", "field2"]);
}
