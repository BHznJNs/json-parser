mod utils;

use json_parser::*;

#[test]
fn array_value_test() {
    let source_vec = vec![
        "[true]",
        "[1, 2, 3]"
    ];
    let expected_vec = vec![
        Value::Array(vec![Value::Bool(true)]),
        Value::Array(vec![
            Value::Number(Number::Int(1)),
            Value::Number(Number::Int(2)),
            Value::Number(Number::Int(3)),
        ]),
    ];
    utils::source_expected_pair_test(source_vec, expected_vec);
}