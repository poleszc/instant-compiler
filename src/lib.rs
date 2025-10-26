// Export all modules used by the compiler from the module for them to be used in tests.
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub instant);

pub mod ast;
pub mod error;
pub mod lexer;
pub mod tokens;

pub use crate::instant::ProgramParser;
pub use crate::lexer::MyLexer;
