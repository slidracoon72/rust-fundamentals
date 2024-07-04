mod borrowing;
mod error_handling;
mod structs;

use std::{collections::HashMap, io, os::fd::AsRawFd};

fn print_demo() {
    println!("Hello, world!");
    println!("I am Rahul Trivedi!");
    println!("Rust is a systems programming language focused on safety and performance");
}

fn shadowing() {
    // Original integer variable declaration
    let x = 42;
    println!("x: {}", x);

    // Variable reassignment (shadowing) within the same scope
    let x = "forty-two";
    println!("x: {}", x);

    {
        // Creating a nested scope where 'x' has a new binding
        let x = 42.5;
        println!("x: {}", x);

        // Leaving the inner scope - original bindings restored
    }

    // Shadowing variables
    let mut height = 190;
    height = height - 20;
    let result = if height > 180 {
        "Tall"
    } else if height >= 170 {
        "Average"
    } else {
        "Short"
    };
    println!("Height: {}, Result: {}", height, result);

    // shadowing to a diffrent type
    let height = if height > 170 { true } else { false };
    println!("Height: {}", height);
}

fn variables() {
    let message = "Name: Rahul Trivedi";
    let weight = 170.0;
    let kilos = weight / 2.2;
    println!("Message: {}, Weight: {}, Kilos: {}", message, weight, kilos);
}

// to updata the same variable, use 'mut'
fn mutability() {
    let mut message = "Hello, world!";
    message = "Goodbye, world!";
    println!("{}", message);
}

// create a function that takes a string as input and returns the string reversed
fn reverse_string(input: String) -> String {
    let mut reversed = String::new();
    for c in input.chars().rev() {
        reversed.push(c);
    }
    // return reversed;
    reversed
}

fn conditional() {
    // if-else
    let proceed = false;
    if proceed {
        println!("Proceeding!");
    } else {
        println!("Not proceeding");
    }

    let name = "Rahuls";
    if name == "Rahul" {
        println!("Hello, Rahul!");
    } else {
        println!("Hello, {}", name);
    }

    // if-else if-else
    let x = -5;
    if x > 0 {
        println!("x is positive");
    } else if x < 0 {
        println!("x is negative");
    } else {
        println!("x is zero");
    }

    // In-Line conditionals
    let obesity = 200;
    let health = if obesity > 180 { "Poor" } else { "Good" };
    println!("Health: {}", health);
}

fn options() {
    // Nested Optionals with a number
    let maybe_number = Some(42); // or None in some cases
    if let Some(num) = maybe_number {
        println!("The number is {}", num);
    } else {
        println!("No number provided.");
    }

    // Nested Optionals with None
    let maybe_name: Option<Option<()>> = Some(None);
    if let Some(name) = maybe_name {
        println!("The name is {:?}", name);
    } else {
        println!("No name provided");
    }
}

fn loop_demo() {
    let mut x = 0;
    loop {
        println!("Loop-x is: {}", x);
        x += 1;
        if x == 5 {
            break;
        }
    }
}

fn while_loop() {
    let mut x = 0;
    while x < 5 {
        println!("While-x is: {}", x);
        x += 1;
    }

    let mut input = String::new();
    while input.trim() != "Stop" {
        input.clear();
        println!("Please enter a word (type 'Stop' to exit the loop:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("You entered: {}", input.trim());
    }
    println!("Exiting the loop!");
}
fn for_loop() {
    for i in 0..5 {
        println!("For-i is: {}", i);
    }
    println!("* * * * *");

    for x in (0..=5).rev() {
        println!("For-x is: {}", x);
    }
    println!("* * * * *");

    let numbers = vec![5, 6, 7, 8, 9];
    for num in numbers {
        println!("For-num is: {}", num);
    }
    println!("* * * * *");

    // skip even numbers - continue and break
    for i in 0..=100 {
        if i % 2 == 0 {
            continue;
        }
        println!("i is: {}", i);
        if i == 7 {
            break;
        }
    }
}

fn match_demo() {
    let name = "Rahul";
    match name {
        "John" => println!("John"),
        "Rahul" => println!("Hello, Rahul!"),
        _ => println!("Hello, {}!", name),
    }

    let number = 5;
    match number {
        1..=10 => println!("Between 1 and 10"),
        _ => println!("Outside the range"),
    }

    println!("Please enter a greeting:");
    let mut greet = String::new();
    while greet.trim() != "Stop" {
        greet.clear();
        println!("Please enter a greeting (type 'Stop' to exit the loop:");
        io::stdin()
            .read_line(&mut greet)
            .expect("Failed to read line");

        match greet.trim().to_lowercase().as_str() {
            "hello" => println!("Hello!"),
            "hi" => println!("Hi!"),
            _ => println!("No greeting recognized."),
        }
    }
}

// create a function that uses arrays
fn arrays_demo() {
    // create an array of integers
    let arr = [1, 2, 3, 4, 5];
    println!("The array is: {:?}", arr);

    // access array elements
    println!("The first element is: {}", arr[0]);

    // iterate over the array elements
    for i in 0..arr.len() {
        println!("Element at index {} is: {}", i, arr[i]);
    }

    // create an array of strings
    let names = ["John", "Jane", "Bob"];
    println!("The names array is: {:?}", names);

    // access array elements
    println!("The first name is: {}", names[0]);

    // iterate over the array elements
    for i in 0..names.len() {
        println!("Name at index {} is: {}", i, names[i]);
    }
}

// create a function that uses vectors
fn vectors_demo() {
    // create a vector of integers
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("The vector is: {:?}", vec);

    // access vector elements
    println!("The first element is: {}", vec[0]);

    // iterate over the vector elements
    for i in 0..vec.len() {
        println!("Element at index {} is: {}", i, vec[i]);
    }

    // create a vector of strings
    let mut names = Vec::new();
    names.push("John".to_string());
    names.push("Jane".to_string());
    names.push("Bob".to_string());
    println!("The names vector is: {:?}", names);

    // access vector elements
    println!("The first name is: {}", names[0]);

    // iterate over the vector elements
    for i in 0..names.len() {
        println!("Name at index {} is: {}", i, names[i]);
    }
}

// create a function that uses Hash Map Data Type
fn hash_map_demo() {
    // create a new HashMap
    let mut map = HashMap::new();

    // insert key-value pairs into the HashMap
    map.insert("name", "John");
    map.insert("age", "30");
    map.insert("city", "New York");

    // create more key-value pairs
    map.insert("name", "Rahul");
    map.insert("age", "25");
    map.insert("city", "London");

    // retrieve a value from the HashMap
    let name = map.get("name").unwrap();
    println!("Name: {}", name);

    // check if a key exists in the HashMap
    if let Some(age) = map.get("age") {
        println!("Age: {}", age);
    }

    // remove a key-value pair from the HashMap
    map.remove("city");

    // iterate over the key-value pairs in the HashMap
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}

// functions in Rust

// a unit function that doesn't return anything
fn print_sum(numbers: &[i32]) {
    let sum: i32 = numbers.iter().sum(); // Calculate the sum of elements in slice
    if sum % 2 == 0 {
        // Check if sum is even
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
}

// Using panic macro
fn process_numbers(slice: &[i32]) {
    for (index, number) in slice.iter().enumerate() {
        if *number < 0 {
            panic!("Negative number found at index {}", index); // Stop execution and show error message
        }
    }
}

fn loop_and_panic(numbers: Vec<i32>) {
    for number in numbers {
        if number < 0 {
            panic!("Negative number found: {}", number);
        }
        println!("Number: {}", number);
    }
}

// functions with return types
fn split_string(s: String, delimiter: char, index: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(index);

    result.expect("Something went wrong!").to_string()
}

// Option as a type
fn option(){
        let age: Option<u8> = Some(25);
        match age {
            Some(x) => println!("Age is {}", x),
            None => println!("No age provided"),
        }
}

fn main() {
    print_demo();
    println!("--------------");
    let input = "Reverse this string!";
    let reversed = reverse_string(input.to_string());
    println!("The reversed string is: {}", reversed);
    println!("--------------");
    shadowing();
    println!("--------------");
    variables();
    println!("--------------");
    mutability();
    println!("--------------");
    conditional();
    println!("--------------");
    options();
    println!("--------------");
    loop_demo();
    println!("--------------");
    // while_loop();
    println!("--------------");
    for_loop();
    println!("--------------");
    // match_demo();
    println!("--------------");
    arrays_demo();
    println!("--------------");
    vectors_demo();
    println!("--------------");
    hash_map_demo();
    println!("--------------");
    let numbers = [1, 2, 3];
    print_sum(&numbers);
    println!("--------------");
    let numbers = [1, 2, 3, -5];
    // process_numbers(&numbers);
    println!("--------------");
    let chunk = split_string("Hello, World, Rahul".to_string(), ',', 2);
    println!("The chunk is: {}", chunk);
    println!("--------------");
    option();
    println!("--------------");
    structs::use_struct();
    println!("--------------");
    borrowing::borrow();
    println!("--------------");
    // loop_and_panic(vec![1, 2, 3, 4, -5]);
    println!("--------------");
    error_handling::handle_error();
    println!("--------------");
    
}
