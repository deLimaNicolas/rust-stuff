use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("Guess the number!");

    println!("Input your guess buddy");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You Guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("You guessed less buddy"),
        Ordering::Equal => println!("Nice! That was a match"),
        Ordering::Greater => println!("Too high, try again..."),
    }
}
