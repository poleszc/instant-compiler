use clap::Parser;
use instant_compiler::driver::run_compiler;
use instant_compiler::jvm::compile_jasmine;
use instant_compiler::report_error;

fn main() {
    eprintln!("[JVM Backend] Running Instant Compiler");
    if let Err(err) = run_compiler(compile_jasmine) {
        // TODO: Try to get rid of that?
        let args = instant_compiler::InstantArgs::parse();
        let source_code = std::fs::read_to_string(&args.input_file).unwrap_or_default();
        report_error(args.input_file.to_str().unwrap(), &source_code, err);
        eprintln!("\n[JVM Backend] Compilation failed");
        std::process::exit(1);
    }
    eprintln!("[JVM Backend] Compilation Successful");
}
