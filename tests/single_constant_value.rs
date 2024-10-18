mod utils;

#[test]
fn single_constant_value_test() {
    let source_vec = vec![
        "true",
        "false",
        "null",
    ];
    let expected_vec = vec![
        json_parser::Value::Bool(true),
        json_parser::Value::Bool(false),
        json_parser::Value::Null,
    ];
    utils::source_expected_pair_test(source_vec, expected_vec);
}