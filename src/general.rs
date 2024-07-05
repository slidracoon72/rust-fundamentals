pub fn print_demo() {
    println!("Hello, world!");
    println!("I am Rahul Trivedi!");
    println!("Rust is a systems programming language focused on safety and performance");
}

pub fn shadowing() {
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

pub fn variables() {
    let message = "Name: Rahul Trivedi";
    let weight = 170.0;
    let kilos = weight / 2.2;
    println!("Message: {}, Weight: {}, Kilos: {}", message, weight, kilos);
}

// to updata the same variable, use 'mut'
pub fn mutability() {
    let mut message = "Hello, world!";
    message = "Goodbye, world!";
    println!("{}", message);
}

// functions in Rust

// a unit function that doesn't return anything
pub fn print_sum(numbers: &[i32]) {
    let sum: i32 = numbers.iter().sum(); // Calculate the sum of elements in slice
    if sum % 2 == 0 {
        // Check if sum is even
        println!("The sum is even.");
    } else {
        println!("The sum is odd.");
    }
}

// create a function that takes a string as input and returns the string reversed
pub fn reverse_string(input: String) -> String {
    let mut reversed = String::new();
    for c in input.chars().rev() {
        reversed.push(c);
    }
    // return reversed;
    reversed
}

pub fn options() {
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

// Option as a type
pub fn option() {
    let age: Option<u8> = Some(25);
    match age {
        Some(x) => println!("Age is {}", x),
        None => println!("No age provided"),
    }
}
