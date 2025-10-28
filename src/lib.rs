// Export all modules used by the compiler from the module for them to be used in tests.
use clap::Parser;
use lalrpop_util::lalrpop_mod;
use std::path::PathBuf;

lalrpop_mod!(pub instant);

pub mod ast;
pub mod diagnostics;
pub mod error;
pub mod lexer;
pub mod tokens;

#[cfg(feature = "jvm_backend")]
pub mod jvm;
#[cfg(feature = "llvm_backend")]
pub mod llvm;

pub use crate::ast::SourcePosition;
pub use crate::diagnostics::report_error;
pub use crate::error::LoweringError;
pub use crate::instant::ProgramParser;
pub use crate::lexer::MyLexer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct InstantArgs {
    /// Input file for the compiler.
    pub input_file: PathBuf,
    /// Run the program after successful compilation.
    #[arg(short, long)]
    pub run: bool,
    /// Output file name.
    /// If not specified, name of the file matches the input file.
    #[arg(short, long)]
    pub output: Option<String>,
    /// Output directory.
    /// If not specified, produces the output files in the same directory as the source code.
    #[arg(long, value_name = "DIR")]
    pub output_dir: Option<PathBuf>,
}
