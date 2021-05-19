use rand::Rng;
use std::cmp::Ordering;
// io library from the standard library
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
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

        // shadow the int guess onto the existing string guess
        // u32: unsigned, 32-bit integer (good default for small, positive numbers)
        let guess: u32 = guess
            // removes white space from beginning and end
            .trim()
            // parses string into a u32 (specified above)
            .parse()
            .expect("Please type a number!");

        // print user input
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
