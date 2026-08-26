use std::io;

use rand::Rng;


fn main() {

    println!("Guess a random number!");

    let mut randomNumber = rand::thread_rng().gen_range(1..=100); // generates rand number 1-100

    println!("The random number is: {randomNumber}"); // prints rand number;

    // loop create infinite loop that runs until a break
    loop {
        println!("Please enter guess");

        let mut guessString = String::new(); // create new empty String object

        io::stdin().read_line(&mut guessString).expect("Failed to read line"); // reads guess as string

        let guessNum: i32 = guessString.trim().parse().expect("Please enter number"); // trims string and casts to i32 number

        // 
        if (guessNum == randomNumber) {
            println!("You guessed the number!");
            break;
        } else {
            println!("Sorry wrong guess");
        }
    }
}