use std::io;
use std::io::Write;

fn main() {

    // declares fixed array size of 3 i32 integers
    // inits all three slots to 0
    let mut array:[i32; 3] = [0; 3];

    println!("Please enter 3 numbers");
    for i in 0..3 {
        print!("Enter a number: ");
        io::stdout().flush().expect("Failed to flush line buffer");
        let mut user_input = String:: from("");
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let number: i32 = user_input.trim().parse().expect("Input isn't a number");

        array[i] = number;
    }

    println!();

    for val in array {
        print!("{val} ");
    }
    println!();

    let largest_number = largest(array);

    println!("The largest number is: {largest_number}");
}

fn largest(passed_array: [i32; 3]) -> i32 {
    let mut largest_value = passed_array[0];

    for val in passed_array {
        if val >= largest_value {
            largest_value = val;
        }
    }
    largest_value
}