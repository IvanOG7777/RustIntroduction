use std::io;
fn main() {
    let mut user_input = String::from("");

    println!("Please enter a number");
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let number: i32 = user_input.trim().parse().expect("Value isn't a number");

    println!("Is passed value positive: {}", is_positive(number));
}

fn is_positive(passed_number: i32) -> bool {
    passed_number >=0
}