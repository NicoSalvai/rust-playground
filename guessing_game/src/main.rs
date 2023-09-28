use rand::Rng;
use std::cmp::Ordering;
use std::io; // Importing standard IO crate

fn main() {
    // Declare and initialize a final variable;
    let secret_number = rand::thread_rng().gen_range(1..=100); // 1..=100 is a range -> is inclusive in both ends

    println!("Guess the number!");

    loop {
        // Declare and initialize a mutable variable -> Type is going to be String due to type inference
        let mut guess = String::new();

        println!("Please input a number:");

        // Read input from standard I.
        io::stdin()
            .read_line(&mut guess) // read a line
            .expect("Failed to read line"); // if anything fails show this error message first.

        // Shadowing -> allows us to reuse a variable name and change its infered type.
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Wrong input!");
                continue;
            },
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!"); 
                break;
            },
        }
    }
}
