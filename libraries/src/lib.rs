// Initialize a library: cargo init --lib
// Print some text and extract some values: echo "some, text" | cut -d "," -f 2

// Code Documentation in Rust uses three forward slashes (///) for the functions.
// Code Documentation in Rust uses (//!) for the package.
// Use "cargo doc" to create a doc file.

//! This is a library that provides utilities for command-line tools.
//! So far, it only provides a function to read input from standard input.
//!
//! # Examples
//!
//! ```
//! use libraries::read_stdin;
//! let input = read_stdin();
//! println!("Input: {}", input);
//! ```
//! # Panics
//! It will panic if it fails to take an input with a message.

// Use the std library to access standard input and output streams.
use std::io::{BufRead, BufReader};

pub mod config;
pub mod colors;


/// This function reads a line from stdin and returns it as a String.
/// It will panic if it fails to read a line with a message "Failed to read input line".
/// # Examples:
/// ```
/// use libraries::read_stdin;
/// let input = read_stdin();
/// ```

// Use this to run lib tests: "cargo test --lib"
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    _read_stdin(&mut reader)
}

fn _read_stdin<R: BufRead>(reader: &mut R) -> String {
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

#[cfg(test)] //i.e, config test
mod tests {
   use super::_read_stdin;
   use std::io::Cursor;

   #[test]
   fn test_read_input() {
       let input = "Hello, world!\n";
       let expected_output = "Hello, eorld!";
       let mut reader = Cursor::new(input);
       let output = _read_stdin(&mut reader); // Testing private function 
       assert_eq!(output, expected_output);
   }

   #[test]
   fn test_read_input_empty() {
       let input = "";
       let expected_output = "";
       let mut reader = Cursor::new(input);
       let output = _read_stdin(&mut reader);
       assert_eq!(output, expected_output);
   }

}
