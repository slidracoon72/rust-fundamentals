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
use std::io::stdin;
use std::io::{BufRead, BufReader};

/// This function reads a line from standard input and returns it as a string.
/// It uses a BufReader to read from standard input efficiently.
/// It will panic if it fails to take an input with a message.
/// # Examples:
/// ```
/// let input = read_stdin();
/// ```
pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader
        .read_line(&mut line)
        .expect("Failed to read input line");
    line.trim().to_string()
}

fn main() {
    let input = read_stdin();
    println!("Input: {}", input);
}
