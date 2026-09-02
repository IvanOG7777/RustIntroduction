use std::ops::Add;

fn main() {
    let mut message = String::from("Hello, World");

    println!("Before");
    println!("{message}");

    println!();
    add_exclamation(&mut message);

    println!("After");
    println!("{message}");
}

fn add_exclamation(passed_string: &mut String) {
    passed_string.push_str("!");
}