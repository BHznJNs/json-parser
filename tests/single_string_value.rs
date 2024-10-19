mod utils;

use json_parser::*;

#[test]
fn single_string_value_test() {
    let source_vec = vec![
        "\"\"", // empty string
        "\"Hello World!\"", // common content

        // escape character
        "\"\\\"\"", "\"\\/\"", "\"\\\\\"",
        "\"\\b\"", "\"\\f\"", "\"\\n\"", "\"\\r\"", "\"\\t\"",
        
        "\"Hello\\nWorld\"", // content with escape character
    ];
    let expected_vec = vec![
        Value::String("".to_string()),
        Value::String("Hello World!".to_string()),

        Value::String("\"".to_string()),
        Value::String("/".to_string()),
        Value::String("\\".to_string()),
        Value::String("\x08".to_string()),
        Value::String("\x0C".to_string()),
        Value::String("\n".to_string()),
        Value::String("\r".to_string()),
        Value::String("\t".to_string()),

        Value::String("Hello\nWorld".to_string()),
    ];
    utils::source_expected_pair_test(source_vec, expected_vec);
}
