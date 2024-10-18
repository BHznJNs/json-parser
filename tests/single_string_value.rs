mod utils;

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
        json_parser::Value::String("".to_string()),
        json_parser::Value::String("Hello World!".to_string()),

        json_parser::Value::String("\"".to_string()),
        json_parser::Value::String("/".to_string()),
        json_parser::Value::String("\\".to_string()),
        json_parser::Value::String("\x08".to_string()),
        json_parser::Value::String("\x0C".to_string()),
        json_parser::Value::String("\n".to_string()),
        json_parser::Value::String("\r".to_string()),
        json_parser::Value::String("\t".to_string()),

        json_parser::Value::String("Hello\nWorld".to_string()),
    ];
    utils::source_expected_pair_test(source_vec, expected_vec);
}
