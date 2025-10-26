use lalrpop_util::lalrpop_mod;
use lexer::Lexer;

lalrpop_mod!(pub instant);

pub mod ast;
pub mod lexer;
pub mod tokens;

use crate::instant::ProgramParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, Instant!");

    let source_code = std::fs::read_to_string("tests/in/test07.ins")?;
    let lexer = Lexer::new(&source_code);
    let parser = ProgramParser::new();
    let ast = parser.parse(lexer)?;

    println!("{:?}", ast);
    Ok(())
}
