use clap::Parser;
use instant_compiler::driver::run_compiler;
use instant_compiler::llvm::compile_llvm;
use instant_compiler::report_error;

fn main() {
    // TODO: Add debug feature flag, and print only then.
    println!("[LLVM Backend] Running Instant Compiler");
    if let Err(err) = run_compiler(compile_llvm) {
        // TODO: Try to get rid of that?
        let args = instant_compiler::InstantArgs::parse();
        let source_code = std::fs::read_to_string(&args.input_file).unwrap_or_default();
        report_error(args.input_file.to_str().unwrap(), &source_code, err);
        println!("\n[LLVM Backend] Compilation failed");
        std::process::exit(1);
    }
    println!("[LLVM Backend] Compilation Successful");
}
