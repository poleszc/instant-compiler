use instant_compiler::{ProgramParser, MyLexer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Instant!");

    let source_code = std::fs::read_to_string("tests/in/test06.ins")?;
    let lexer = MyLexer::new(&source_code);
    let parser = ProgramParser::new();
    let ast = parser.parse(lexer)?;

    println!("{:?}", ast);
    Ok(())
}
