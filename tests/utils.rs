pub fn source_expected_pair_test(source_vec: Vec<&str>, expected_vec: Vec<json_parser::Value>) {
    for (&source, expected) in source_vec.iter().zip(expected_vec.iter()) {
        let result = json_parser::parse(source);
        assert!(result.is_ok(), "source: {}\nerror: {}", source, result.unwrap_err().error_msg());
        let value = result.unwrap();
        assert_eq!(value, expected.clone(), "source: {}", source);
    }
}
