use std::io;
use std::io::Write;

const MAX_NUMS: usize = 5;
fn main() {
    let mut array:[i32; MAX_NUMS] = [0; MAX_NUMS];

    println!("Please enter: {MAX_NUMS} numbers");
    for i in 0..MAX_NUMS {
        let mut user_input = String::new();

        print!("Enter number: ");
        io::stdout().flush().expect("Failed to flush line buffer");

        io::stdin().read_line(&mut user_input).expect("Failed to read line");

        let number: i32 = user_input.trim().parse().expect("Value isn't a number");

        array[i] = number;
    }

    let mut user_input = String:: from("");
    println!("What would you like to find with new array");
    println!("1: Sum");
    println!("2: Average");
    println!("3: Minimum");
    println!("4: Maximum");

    print!("Enter option: ");
    io::stdout().flush().expect("Failed to flush line buffer");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let option: i32 = user_input.trim().parse().expect("Value isn't a number");

    match option {
        1 => println!("Sum is: {}", array_sum(array)),
        2 => println!("Average is: {}", array_average(array)),
        3 => println!("Minimum is: {}", array_minimum(array)),
        4 => println!("Maximum is: {}", array_maximum(array)),
        _=>println!("Invalid input"),
    }
}

fn array_sum(passed_array: [i32; MAX_NUMS]) -> i32 {
    let mut sum = 0;

    for val in passed_array {
        sum += val;
    }

    sum
}

fn array_average(passed_array: [i32; MAX_NUMS]) -> i32 {
    let mut sum = 0;
    let average: i32;

    for val in passed_array {
        sum += val;
    }

    average = sum / (MAX_NUMS as i32);

    average
}

fn array_minimum(passed_array: [i32; MAX_NUMS]) -> i32 {
    let mut minimum = passed_array[0];

    for val in passed_array {
        if val <= minimum {
            minimum = val;
        }
    }

    minimum
}

fn array_maximum(passed_array: [i32; MAX_NUMS]) -> i32 {
    let mut maximum = passed_array[0];

    for val in passed_array {
        if val >= maximum {
            maximum = val;
        }
    }

    maximum
}