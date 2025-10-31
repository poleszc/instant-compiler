use crate::InstantArgs;
use crate::error::CompilerError;
use crate::llvm::LLVMGenerator;
use crate::parser::ast::Program;
use std::{fs, path::Path, process::Command};

pub fn compile_llvm(
    program: &mut Program,
    dir: &Path,
    stem: &str,
    args: &InstantArgs,
) -> Result<(), CompilerError> {
    println!("[LLVM] Generating LLVM IR code...");

    let ll_path = dir.join(format!("{}.ll", stem));
    let bc_path = dir.join(format!("{}.bc", stem));
    let exe_path = dir.join(stem);
    let mut generator = LLVMGenerator::new();
    let llvm_code = generator.lower_program(program)?;

    fs::write(&ll_path, llvm_code)?;
    println!("[LLVM] Generated file: {}", ll_path.display());

    println!("[LLVM] Assembling bitcode...");
    let output = Command::new("llvm-as")
        .arg(&ll_path)
        .arg("-o")
        .arg(&bc_path)
        .output()?;

    if !output.status.success() {
        return Err(CompilerError::ExternalCommand(format!(
            "LLVM assembler failed.\n[STDERR]:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    println!("[LLVM] Generated file: {}", bc_path.display());

    if args.run {
        println!("[LLVM] Running program");
        let run_output = Command::new("lli").arg(&bc_path).output()?;

        if !run_output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&run_output.stdout))
        }

        if !run_output.stderr.is_empty() {
            println!("{}", String::from_utf8_lossy(&run_output.stderr))
        }

        if !run_output.status.success() {
            return Err(CompilerError::ExternalCommand(
                "Program execution exited with a non-zero status.".to_string(),
            ));
        }
    } else {
        println!("[LLVM] Linking executable");
        let run_output = Command::new("clang")
            .arg(&bc_path)
            .arg("lib/llvm/runtime.bc")
            .arg("-o")
            .arg(&exe_path)
            .output()?;

        if !run_output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&run_output.stdout))
        }

        if !run_output.stderr.is_empty() {
            println!("{}", String::from_utf8_lossy(&run_output.stderr))
        }

        if !run_output.status.success() {
            return Err(CompilerError::ExternalCommand(
                "Failed to link an executable".to_string(),
            ));
        }
    }

    Ok(())
}
