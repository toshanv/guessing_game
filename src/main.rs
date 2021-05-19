// io library from the standard library
use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // 'let' is used to create a variable
    // variables are immutable by default
    // 'mut' makes the variable mutable
    let mut guess = String::new();

    // get input from user
    io::stdin()
        // 'read_line' appends input to the parameter
        // '&' indicates that the argument is a reference (immutable by default)
        // 'mut' makes the reference mutable
        .read_line(&mut guess)
        .expect("Failed to read line");

    // print user input
    println!("You guessed: {}", guess);
}
