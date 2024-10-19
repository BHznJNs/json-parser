mod utils;

use json_parser::*;

#[test]
fn object_value_test() {
    let source_vec = vec![
        "{\"a\": 1}",
    ];
    let expected_vec = vec![
        Value::Object(Object::from([
            ("a".to_string(), Box::new(Value::Number(Number::Int(1)))),
        ])),
    ];
    utils::source_expected_pair_test(source_vec, expected_vec);
}
