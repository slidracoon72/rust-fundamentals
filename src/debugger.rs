use std::io;

// Using 'CodeLLDB' extension as a debugger.

/// This function prints a greeting message and asks for the user's name.
/// It will continue to ask for names until the user enters "exit".
/// # Examples:
/// ```
/// greetings();
/// ```
pub fn greetings() {
    let mut input = String::new();
    while input.trim().to_lowercase() != "exit" {
        input.clear();
        println!("Enter your name (or 'exit' to quit):");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        if input.trim() != "exit" {
            print!("Hello, {}!", input);
        }
    }
    println!("Goodbye!");
}
