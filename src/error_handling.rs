use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn handle_error() {
    let args: Vec<String> = env::args().collect();

    // Ensure at least one argument (the file path) is provided
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    // $ ./args arg1 arg2
    // ******** do this: $ target/debug/rust-fundamentals demo.txt ***********

    //println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    let file_name = &args[1];
    println!("File name: {}", file_name);

    let file = File::open(file_name);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => { 
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
