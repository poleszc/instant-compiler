// Export ast from the module.
pub mod ast;
mod lexer;
pub mod tokens;

// #[allow(clippy::all)]
// mod instant;

use crate::error::CompilerError;
use crate::instant::ProgramParser;
use lexer::MyLexer;

pub fn parse(source_code: &str) -> Result<ast::Program, CompilerError> {
    let lexer = MyLexer::new(source_code);
    let parser = ProgramParser::new();
    let program = parser.parse(lexer)?;

    Ok(program)
}
