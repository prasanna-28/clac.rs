use std::env;
use std::process::Command;

mod parser;
mod ir;
mod codegen;

fn main() {
    // Get the filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: clac <filename>");
        std::process::exit(1);
    }
    let filename = &args[1];

    let tokens = parser::parse_file(filename);

    let ir_code = ir::convert_to_ir(tokens);

    let asm_code = codegen::generate_assembly(ir_code);

    let asm_filename = "output.asm";
    std::fs::write(asm_filename, asm_code).expect("Failed to write assembly file");

    panic!("nasm and linking should be done manually"); 
    // the code below should do them automatically but unsure so panic for now, maybe usable if linux machine


    let status = Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg(asm_filename)
        .status()
        .expect("Failed to run nasm");

    if !status.success() {
        eprintln!("nasm failed with status: {}", status);
        std::process::exit(1);
    }

    // Link the object file to create an executable
    let status = Command::new("ld")
        .arg("-o")
        .arg("output")
        .arg("output.o")
        .status()
        .expect("Failed to run ld");

    if !status.success() {
        eprintln!("ld failed with status: {}", status);
        std::process::exit(1);
    }

    println!("Compilation succeeded. Executable created: output");
}