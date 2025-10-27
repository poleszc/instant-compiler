use crate::InstantArgs;
use crate::LoweringError;
use crate::ast::Program;
use crate::jvm::JasmineGenerator;
use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(feature = "jvm_backend")]
pub fn compile_jasmine(
    program: &mut Program,
    dir: &Path,
    stem: &str,
    args: &InstantArgs,
) -> Result<(), LoweringError> {
    println!("[JVM] Generating Jasmine code...");

    let jasmine_path = dir.join(format!("{}.j", stem));
    let class_path = dir.join(format!("{}.class", stem));

    let mut generator = JasmineGenerator::new(stem);
    let jasmine_code = generator.lower_program(program).expect("Failed to compile");

    fs::write(&jasmine_path, jasmine_code).expect(
        format!(
            "Failed to save the compiled code to: {}",
            jasmine_path.display()
        )
        .as_str(),
    );
    println!("[JVM] Generated file: {}", jasmine_path.display());

    println!("[JVM] Assembling Jasmine...");
    let status = Command::new("java")
        .arg("-jar")
        .arg("lib/jvm/jasmin.jar")
        .arg("-d")
        .arg(dir)
        .arg(&jasmine_path)
        .status()
        .expect("Failed to assemble jasmine");
    if !status.success() {
        // TODO: This is ugly
        println!("Jasmine exited with error");
        return Ok(());
    }

    println!("[JVM] Generated file: {}", class_path.display());

    if args.run {
        println!("[JVM] Running program");
        let run_status = Command::new("java")
            .arg("-cp")
            .arg(dir)
            .arg(stem)
            .status()
            .expect("Failed to execute program!");
        if !run_status.success() {
            println!("JVM exited with an error");
        }
    }

    Ok(())
}
