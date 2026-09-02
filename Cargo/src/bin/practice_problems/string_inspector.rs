fn main () {

    let message = String::from("I am learning Rust");

    println!("Length of message is: {}", print_length(&message));
    println!("There are: {} spaces", count_spaces(&message));
    println!("Does message contain the word 'Rust'?: {}", contains_rust(&message));
    println!("{message}");
}

fn print_length(passed_string: &String) -> usize {
    passed_string.len()
}

fn count_spaces(passed_string: &String) -> usize {
    let mut spaces: usize = 0;

    for char in passed_string.as_bytes() {
        if *char == b' ' {
            spaces += 1;
        }
    }

    spaces
}

fn contains_rust(passed_string: &String) -> bool {
    let mut contains = false;

    for word in passed_string.split_whitespace() {
        if word == "Rust" || word == "rust" {
            contains = true;
        }
    }

    contains
}