// Rust will bring into scope it's std::prelude
// Things like io need to be explicitly imported.
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // This creates a variable.
    // We set mut, because rust variables are immutable by default.
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess)
}
