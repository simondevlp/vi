use std::{env, fs::File, io::Read, process};

use parser::parser::Parser;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Include a source file!");
        process::exit(1);
    }
    let source_file_name = &args[1];
    let Ok(mut source_file) = File::open(source_file_name) else {
        eprintln!("Could not open source file: {}", source_file_name);
        process::exit(1);
    };
    // source_code should be stored as byte sequence.
    // using string for poc.
    let mut source_code = String::new();
    let Ok(_) = source_file.read_to_string(&mut source_code) else {
        eprintln!("Could not read source file: {}", source_file_name);
        process::exit(1);
    };
    let mut parser = Parser::new(&source_code);
    let Some(programme) = parser.visit_programme() else {
        parser.print_diags();
        process::exit(1);
    };
    println!("Parsed programme: {:#?}", programme);
}
