use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("Guess the number!");

    println!("Input your guess buddy");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You Guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("You guessed less buddy"),
            Ordering::Equal => {
                println!("Nice! That was a match");
                break;
            }
            Ordering::Greater => println!("Too high, try again..."),
        }
    }
}
