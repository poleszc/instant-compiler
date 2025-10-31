use crate::InstantArgs;
use crate::error::CompilerError;
use crate::jvm::JasmineGenerator;
use crate::parser::ast::Program;
use std::{fs, path::Path, process::Command};

pub fn compile_jasmine(
    program: &mut Program,
    dir: &Path,
    stem: &str,
    args: &InstantArgs,
) -> Result<(), CompilerError> {
    eprintln!("[JVM] Generating Jasmine code...");

    let jasmine_path = dir.join(format!("{}.j", stem));
    let class_path = dir.join(format!("{}.class", stem));
    let mut generator = JasmineGenerator::new(stem);
    let jasmine_code = generator.lower_program(program)?;

    fs::write(&jasmine_path, jasmine_code)?;
    eprintln!("[JVM] Generated file: {}", jasmine_path.display());

    eprintln!("[JVM] Assembling Jasmine...");
    let output = Command::new("java")
        .arg("-jar")
        .arg("lib/jvm/jasmin.jar")
        .arg("-d")
        .arg(dir)
        .arg(&jasmine_path)
        .output()?;

    if !output.status.success() {
        return Err(CompilerError::ExternalCommand(format!(
            "Jasmine assembler failed.\n[STDERR]:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    eprintln!("[JVM] Generated file: {}", class_path.display());

    if args.run {
        eprintln!("[JVM] Running program");
        let run_output = Command::new("java")
            .arg("-cp")
            .arg(dir)
            .arg(stem)
            .output()?;

        if !run_output.stdout.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&run_output.stdout))
        }

        if !run_output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&run_output.stderr))
        }

        if !run_output.status.success() {
            return Err(CompilerError::ExternalCommand(
                "Program execution exited with a non-zero status.".to_string(),
            ));
        }
    }

    Ok(())
}
