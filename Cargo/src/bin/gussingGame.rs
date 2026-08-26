use std::io; // imports input/output library

fn main() {
    println!("Guess the number!");

    println!("Please enter number");

    let mut guess = String::new(); // allows for input to be mutable (changeable); String:: new() gives guess an empty instance of the String class


    // call stdin from io library
    // use read_line to reference guess and input user input to guess
    //
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {guess}");


}