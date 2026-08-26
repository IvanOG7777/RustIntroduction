use std::io;

use rand::Rng;


fn main() {

    println!("Guess a random number!");

    let randomNumber = rand::thread_rng().gen_range(1..=100);

    println!("The random number is: {randomNumber}");

    print!("Please enter number: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read number");

    print!("You guessed {guess}");
}