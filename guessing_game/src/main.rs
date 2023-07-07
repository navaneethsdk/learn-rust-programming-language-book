use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // thread_rng selects random number generator local to the current thread of execution and is seeded by the operating system
    // Unless otherwise specified, Rust defaults to an i32 type for secret_number
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // creates infinite loop
    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default
        // The :: syntax indicates that new is an associated function of the String type
        let mut guess = String::new();

        // The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        // references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess
        // The parse method on strings converts a string to another type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // The Ordering type is another enum and has the variants Less, Greater, and Equal
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
