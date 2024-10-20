mod utils;

use json_parser::*;

#[test]
fn single_constant_value_test() {
    let source_vec = vec![
        "true",
        "false",
        "null",
    ];
    let expected_vec = vec![
        Value::Bool(true),
        Value::Bool(false),
        Value::Null,
    ];
    utils::source_expected_pair_test(source_vec, expected_vec);
}