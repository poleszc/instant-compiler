// Export all modules used by the compiler from the module for them to be used in tests.
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub instant);

pub mod ast;
pub mod diagnostics;
pub mod error;
pub mod jvm;
pub mod lexer;
pub mod llvm;
pub mod tokens;

pub use crate::diagnostics::report_error;
pub use crate::instant::ProgramParser;
pub use crate::jvm::JasmineGenerator;
pub use crate::lexer::MyLexer;
