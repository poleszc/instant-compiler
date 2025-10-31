mod driver;
mod instructions;
mod lowering;

use lowering::LLVMGenerator;
// TODO: Not needed?
use instructions::{LLVMInstruction, PointerName, RegisterName};

pub use driver::compile_llvm;
