use instant_compiler::{MyLexer, ProgramParser};

fn assert_parsing_succeeds(path: &str) {
    let source_code = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Unable to read file '{}': {}", path, err));
    let lexer = MyLexer::new(&source_code);
    let parser = ProgramParser::new();

    match parser.parse(lexer) {
        Ok(_) => {}
        Err(err) => {
            panic!("Expected parsing to succeed on file '{}': {}", path, err);
        }
    }
}

fn assert_parsing_fails(path: &str) {
    let source_code = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Unable to read file '{}': {}", path, err));
    let lexer = MyLexer::new(&source_code);
    let parser = ProgramParser::new();

    match parser.parse(lexer) {
        Ok(ast) => {
            panic!("Expected parsing to fail on file '{}': {:?}", path, ast);
        }
        Err(_) => {}
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_parsing_tests.rs"));
