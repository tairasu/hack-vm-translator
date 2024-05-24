// VM to Hack assembler translator

use lib::instructions::parse_file;
use std::env;
use std::process;

mod lib;

fn main() {
    // Capture command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the file path is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Load the file content
    let file_content = match lib::load::load_file(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error loading file {}: {}", file_path, e);
            process::exit(1);
        }
    };

    // Parse the file content
    let parsed_content = parse_file(file_content);

    // Create the new file path with .asm extension
    let new_file_path = file_path.replace(".vm", ".asm");

    // Write the parsed content to the new file
    if let Err(e) = lib::load::write_file(new_file_path, parsed_content) {
        eprintln!("Error writing file: {}", e);
        process::exit(1);
    }
}