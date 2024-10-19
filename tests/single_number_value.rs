mod utils;

use json_parser::*;

#[test]
fn single_number_value_test() {
    let source_vec = vec![
        "16", "-16",   // integer test
        "1.6", "-1.6", // float   test

        // exponential test
        "16e2", "16E2", "1.6e2", "1.6E2",
        // negative exponential test
        "-16e2", "-16E2", "-1.6e2", "-1.6E2",
        "-16e3", "-16E3", "-1.6e3", "-1.6E3",
    ];

    let expected_vec = vec![
        Value::Number(Number::Int(16)),
        Value::Number(Number::Int(-16)),
        Value::Number(Number::Float(1.6)),
        Value::Number(Number::Float(-1.6)),

        Value::Number(Number::Int(16_i64.pow(2))),
        Value::Number(Number::Int(16_i64.pow(2))),
        Value::Number(Number::Float(1.6_f64.powi(2))),
        Value::Number(Number::Float(1.6_f64.powi(2))),

        Value::Number(Number::Int((-16_i64).pow(2))),
        Value::Number(Number::Int((-16_i64).pow(2))),
        Value::Number(Number::Float((-1.6_f64).powi(2))),
        Value::Number(Number::Float((-1.6_f64).powi(2))),

        Value::Number(Number::Int((-16_i64).pow(3))),
        Value::Number(Number::Int((-16_i64).pow(3))),
        Value::Number(Number::Float((-1.6_f64).powi(3))),
        Value::Number(Number::Float((-1.6_f64).powi(3))),
    ];

    utils::source_expected_pair_test(source_vec, expected_vec);
}