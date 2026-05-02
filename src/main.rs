mod compiler;
mod lexer;
mod syntax;
mod semantic;

use std::env;
use std::fs;
use std::process;
use compiler::{Compiler, LolCompiler};

fn open_html(filename: &String, compiler: LolCompiler) {
    let output_filename = filename.replace(".lol", ".html");
    let html = compiler.output_stack[0].clone();
    fs::write(&output_filename, html).expect("Failed to write output file");

    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/C", "start", &output_filename])
        .spawn().ok();

    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&output_filename)
        .spawn().ok();

    println!("Compiled to {}", output_filename);
}

fn main() {
    // Get all the arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <input.lol>");
        process::exit(1);
    }

    let filename = &args[1];

    // Check if the file is a .lol file
    if !filename.ends_with(".lol") {
        eprintln!("Error: Input file must have a .lol extension.");
        process::exit(1);
    }

    // Save the whole file into a variable to break down
    let source_code = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file {}: {}", filename, e);
            process::exit(1);
        }
    };

    // Initialize and run the compiler then run then open
    let mut compiler = LolCompiler::new();

    // Compile and give it the whole file
    compiler.compile(&source_code);
    println!("Compiled successfully!");

    // Open the html file
    open_html(filename, compiler);
}