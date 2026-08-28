use std::path::Component::ParentDir;

fn main() {

    let string = String::from("HelloWorld from");
    let first_word = find_first_word(&string); // should return "HelloWorld";

    println!("First word within string is {first_word}");
}

fn find_first_word(passed_string: &String) -> String {
    let mut found_space = false;

    let mut index = 0;
    for char in passed_string.chars() {
        if found_space == true {
            break;
        }
        if char == ' ' {
            found_space = true;
        }

        index += 1;
    }

    println!("Found first space at {index}");

    let s = String::from(&passed_string[0..index]);

    s
}