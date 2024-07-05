use std::io;

pub fn loop_demo() {
    let mut x = 0;
    loop {
        println!("Loop-x is: {}", x);
        x += 1;
        if x == 5 {
            break;
        }
    }
}

pub fn while_loop() {
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

pub fn for_loop() {
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
