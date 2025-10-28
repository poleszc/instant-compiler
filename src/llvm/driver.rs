use crate::InstantArgs;
use crate::LoweringError;
use crate::ast::Program;
use crate::llvm::LLVMGenerator;
use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(feature = "llvm_backend")]
pub fn compile_llvm(
    program: &mut Program,
    dir: &Path,
    stem: &str,
    args: &InstantArgs,
) -> Result<(), LoweringError> {
    println!("[LLVM] Generating LLVM IR code...");

    let ll_path = dir.join(format!("{}.ll", stem));
    let bc_path = dir.join(format!("{}.bc", stem));
    let exe_path = dir.join(stem);

    let mut generator = LLVMGenerator::new();
    let llvm_code = generator.lower_program(program)?;

    fs::write(&ll_path, llvm_code)
        .expect(format!("Failed to save the compiled code to: {}", ll_path.display()).as_str());
    println!("[LLVM] Generated file: {}", ll_path.display());

    println!("[LLVM] Assembling bitcode...");
    let status = Command::new("llvm-as")
        .arg(&ll_path)
        .arg("-o")
        .arg(&bc_path)
        .status()
        .expect("Failed to assemble LLVM bitcode");
    if !status.success() {
        // TODO: This is ugly
        println!("llvm-as exited with error");
        return Ok(());
    }

    println!("[LLVM] Generated file: {}", bc_path.display());

    if args.run {
        println!("[LLVM] Running program");
        let status = Command::new("lli")
            .arg(&bc_path)
            .status()
            .expect("Failed to run LLVM bitcode");
        if !status.success() {
            // TODO: This is ugly
            println!("lli exited with error");
            return Ok(());
        }
    } else {
        println!("[LLVM] Linking executable");
        let status = Command::new("clang")
            .arg(&bc_path)
            .arg("lib/llvm/runtime.bc")
            .arg("-o")
            .arg(&exe_path)
            .status()
            .expect(format!("Failed to link a library: {}", exe_path.display()).as_str());
        if !status.success() {
            // TODO: This is ugly
            println!("lli exited with error");
            return Ok(());
        }
    }

    Ok(())
}
