use std::{env, fs::File, io::Read, process};

use interp::Evaluator;

enum Operation {
    Parse,
    Interpret,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        eprintln!("Specify an operation and include a source file!");
        process::exit(1);
    }
    let operation = match args[1].as_str() {
        "parse" => Operation::Parse,
        "interpret" => Operation::Interpret,
        _ => {
            eprintln!("Unknown operation: {}", args[1]);
            process::exit(1);
        }
    };
    let source_file_name = &args[2];
    let Ok(mut source_file) = File::open(source_file_name) else {
        eprintln!("Could not open source file: {}", source_file_name);
        process::exit(1);
    };
    let mut source_code = String::new();
    let Ok(_) = source_file.read_to_string(&mut source_code) else {
        eprintln!("Could not read source file: {}", source_file_name);
        process::exit(1);
    };
    match operation {
        Operation::Parse => {
            let mut evaluator = Evaluator::new(&source_code);
            let prog = evaluator.parse();
            match prog {
                Some(p) => {
                    println!("Parsed successfully: {:#?}", p);
                }
                None => {
                    eprintln!("Parsing failed.");
                    process::exit(1);
                }
            }
        }
        Operation::Interpret => {
            let mut evaluator = Evaluator::new(&source_code);
            let prog = evaluator.parse();
            evaluator.interpret(&prog);
        }
    }
}
