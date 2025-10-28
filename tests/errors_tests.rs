use instant_compiler::{MyLexer, ProgramParser};

#[cfg(feature = "jvm_backend")]
use instant_compiler::jvm::JasmineGenerator;
#[cfg(feature = "llvm_backend")]
use instant_compiler::llvm::LLVMGenerator;

fn assert_lowering_succeeds(path: &str) {
    let source_code = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Unable to read file '{}': {}", path, err));
    let lexer = MyLexer::new(&source_code);
    let parser = ProgramParser::new();

    let mut program = parser
        .parse(lexer)
        .unwrap_or_else(|err| panic!("Parsing failed: {}", err));

    #[cfg(feature = "jvm_backend")]
    {
        let jasmine_code = JasmineGenerator::new("x").lower_program(&mut program);
        assert!(jasmine_code.is_ok());
    }

    #[cfg(feature = "llvm_backend")]
    {
        let llvm_code = LLVMGenerator::new().lower_program(&mut program);
        assert!(llvm_code.is_ok());
    }
}

fn assert_lowering_fails(path: &str) {
    let source_code = std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("Unable to read file '{}': {}", path, err));
    let lexer = MyLexer::new(&source_code);
    let parser = ProgramParser::new();

    let mut program = parser
        .parse(lexer)
        .unwrap_or_else(|err| panic!("Parsing failed: {}", err));

    #[cfg(feature = "jvm_backend")]
    {
        let jasmine_code = JasmineGenerator::new("x").lower_program(&mut program);
        assert!(jasmine_code.is_err());
    }
    #[cfg(feature = "llvm_backend")]
    {
        let llvm_code = LLVMGenerator::new().lower_program(&mut program);
        assert!(llvm_code.is_err());
    }
}

include!(concat!(env!("OUT_DIR"), "/generated_errors_tests.rs"));
