use clap::Parser;
use std::{fs, path::Path};

use instant_compiler::{InstantArgs, MyLexer, ProgramParser, report_error, report_semantic_error};

#[cfg(feature = "jvm_backend")]
use instant_compiler::jvm::compile_jasmine;

#[cfg(feature = "llvm_backend")]
use instant_compiler::llvm::compile_llvm;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = InstantArgs::parse();

    let output_dir = match args.output_dir {
        Some(ref path) => path.clone(),
        None => args
            .input_file
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf(),
    };

    if let Err(err) = fs::create_dir_all(&output_dir) {
        return Err(format!(
            "Error when creating a directory '{}': {}",
            output_dir.display(),
            err
        )
        .into());
    }

    let input_file = args.input_file.clone();
    let source_code = match fs::read_to_string(&input_file) {
        Ok(code) => code,
        Err(err) => {
            return Err(format!(
                "Error when reading file '{}': {}",
                input_file.display(),
                err
            )
            .into());
        }
    };

    let lexer = MyLexer::new(&source_code);

    let mut program = match ProgramParser::new().parse(lexer) {
        Ok(program) => program,
        Err(err) => {
            report_error(
                input_file
                    .to_str()
                    .expect("Failed to convert input path to string"),
                &source_code,
                err,
            );
            return Err("Parsing failed".into());
        }
    };

    let file_stem = args
        .output
        .as_deref()
        .or_else(|| args.input_file.file_stem().and_then(|s| s.to_str()))
        .expect(
            format!(
                "Could not determine output file name from: {}",
                input_file.display()
            )
            .as_str(),
        );

    #[cfg(feature = "jvm_backend")]
    {
    match compile_jasmine(&mut program, &output_dir, file_stem, &args) {
        Ok(_) => (),
        Err(err) => {
            report_semantic_error(
                input_file
                    .to_str()
                    .expect("Failed to convert input path to string"),
                &source_code,
                err,
            );
            return Err("Parsing failed".into());
            
        }
    }
    }

    #[cfg(feature = "llvm_backend")]
    compile_llvm(&mut program, &output_dir, file_stem, &args).expect("LLVM Lowering failed");

    Ok(())
}
