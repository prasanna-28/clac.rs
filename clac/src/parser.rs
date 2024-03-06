pub fn parse_file(filename: &str) -> Vec<String> {
    let source_code = std::fs::read_to_string(filename).expect("Error Parsing: Failed to read file");

    let tokens: Vec<String> = source_code.split_whitespace().map(|s| s.to_string()).collect();

    tokens
}