//VM to hack assembler translator

use lib::instructions::parse_file;
mod lib;

fn main() {
    let file_path = "7/StackArithmetic/StackTest/StackTest.vm";
    let file_content = lib::load::load_file(file_path).unwrap();
    // println!("{}", parse_file(file_content));

    //export file_content into a new file with the .asm extension with the same name
    let new_file_path = file_path.replace(".vm", ".asm");
    lib::load::write_file(new_file_path, parse_file(file_content)).unwrap();
}
