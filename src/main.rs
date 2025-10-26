use std::fs;

use instant_compiler::{JasmineGenerator, MyLexer, ProgramParser, report_error};

fn main() {
    println!("Hello, Instant!");
    let file_path = "tests/in/test04.ins";
    let source_code = std::fs::read_to_string(file_path).expect("Failed to read a file");
    let lexer = MyLexer::new(&source_code);
    let parser = ProgramParser::new();

    match parser.parse(lexer) {
        Ok(program) => {
            println!("Parsing success!");
            println!("Generating JVM code...");
            let mut generator = JasmineGenerator::new();
            match generator.lower_program(&program) {
                Ok(code) => {
                    let output_path = "output/Main.jasmine";
                    fs::write(output_path, code).expect("File write fail");
                    println!("Generated Jasmine code!");
                }
                Err(err) => {
                    println!("Error when generating code: {:?}", err);
                }
            }
        }
        Err(err) => {
            report_error(file_path, &source_code, err);
        }
    }
}
