use std::{collections::HashMap, io::{BufRead, BufReader}};

// Examples of String and Vectors

pub fn use_string() {
    // String Slice - immutable
    let hello: &str = "Testing Strings!";
    println!("String slice: {}", hello);

    // String Type - mutable and growable
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // Creating a string using the `String::from()` method
    let mut greeting = String::from("Good");
    greeting.push_str(" Morning!");
    println!("String: {}", greeting);
}

pub fn use_vectors() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(44);
    numbers.push(345);
    numbers.push(22);
    println!("Vector: {:?}", numbers);
}

pub fn string_manipulation() {
    let s1 = String::from("Joining");
    let s2 = String::from(" strings!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("String: {}", s3);

    // Slicing
    let sentence = "This is a sentence.".to_string();
    let word = &sentence[5..7];
    println!("{}", word);

    // Concatenate
    let description = format!("Testing concatenating using formats:\n{}", sentence);
    println!("{}", description);

    // Iterate over characters
    let mut count = 0;
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                count += 1;
                println!("{} is a vowel", c)
            }
            _ => continue,
        }
    }
    println!("{} vowels found", count);

    // Count each vowel using HashMap
    let sentence = "Checking for vowels!";

    // Create a HashMap to store the count of each vowel
    let mut vowel_counts = HashMap::new();
    vowel_counts.insert('a', 0);
    vowel_counts.insert('e', 0);
    vowel_counts.insert('i', 0);
    vowel_counts.insert('o', 0);
    vowel_counts.insert('u', 0);

    // Iterate over characters
    for c in sentence.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let count = vowel_counts.entry(c).or_insert(0);
                *count += 1;
                println!("{} is a vowel", c);
            }
            _ => continue,
        }
    }

    // Print the count for each vowel
    for (vowel, count) in &vowel_counts {
        println!("{}: {} times", vowel, count);
    }

    // Split and collect into a vector
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("{:?}", words);

    // Reverse a string
    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
}

// Print longest word in a sentence
fn longest_word(sentence: &str) -> &str {
    let mut longest = "";
    for word in sentence.split_whitespace() {
        if word.len() > longest.len() {
            longest = word;
        }
    }
    longest
}

pub fn longest() {
    let sentence = "This function works by splitting the sentence into words using whitespace as the delimiter.";

    // Find and print the longest word
    let longest = longest_word(sentence);
    println!("The longest word is: {}", longest);
}

// functions with return types
pub fn split_string(s: String, delimiter: char, index: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(index);

    result.expect("Something went wrong!").to_string()
}

// Read Strings from Input/Output
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