use std::io;

pub fn conditional() {
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

pub fn match_demo() {
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
