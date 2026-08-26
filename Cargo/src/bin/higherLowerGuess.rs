use std::cmp::Ordering;
use std::io;
use rand::Rng;
fn main() {
    let randomNumber = rand::thread_rng().gen_range(1..=100); // produces number between 1 and 100
    let mut gameRunning = true; // bool to keep while loop running, set to true

    println!("Playing game 1");
    while (gameRunning) {
        println!("Please enter guess"); // ask for input

        let mut guessString = String::new(); // create new empty String object
        io::stdin().read_line(&mut guessString).expect("Failed to read line"); // read line

        let mut guessNumber: i32 = match guessString.trim().parse() {
            Ok(num) => num, // if guessString.trim.parse produces i32 num within Ok. Ok return that num to guessNumber
            Err(_) => continue, // else dont do anything and continue to top of loop
        }; // cast line to string. currently is non number is entered it break

        // High lower guess prints
        if (guessNumber < randomNumber) {
            println!("You guessed {guessNumber}");
            println!("Too small");
        }
        if (guessNumber > randomNumber) {
            println!("You guessed {guessNumber}");
            println!("Too Big");
        }
        if (guessNumber == randomNumber) {
            println!("Correct Guess!!");
            gameRunning = false;
        }
    }

    gameRunning = true;
    println!("Playing game 2");

    while (gameRunning) {
        println!("Please enter guess");

        let mut guess = String::new(); // create new empty String object

        io::stdin().read_line(& mut guess).expect("Failed to read line"); // read line into guess variable

        // trim and cast guess to 32-bit integer. If it's a num return it else continues
        let guess : i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed {guess}");

        match guess.cmp(&randomNumber) {
            Ordering::Less => println!("Too low"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Correct guess!");
                gameRunning = false;
            }
        }
    }
}