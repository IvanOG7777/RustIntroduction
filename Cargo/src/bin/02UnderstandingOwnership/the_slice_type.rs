use std::path::Component::ParentDir;

fn main() {

    let mut string = String::from("HelloWorld from");
    let first_word = find_first_word(&string); // should return "HelloWorld";

    println!("First word within string is {first_word}");

    string = String::from("Hello world");
    let word = find_first(&string);

    println!("First word within string is: {word}");

    string.clear();

}

// My version
// pass reference to stack allocated variable
fn find_first_word(passed_string: &String) -> String {
    let mut found_space = false;

    let mut index = 0;
    for char in passed_string.chars() { // loop through each char in the referenced string
        if found_space {
            break;
        }
        if char == ' ' {
            found_space = true;
        }

        index += 1;
    }

    println!("Found first space at {index}");

    let s = String::from(&passed_string[0..index]); // slice the old string from index 0 to index counter

    s
}

// Rust documentation style

fn find_first(passed_string:  &String) -> usize {
    let bytes = passed_string.as_bytes(); // get size of passed_string in bytes;

    for (i, &item) in bytes.iter().enumerate() {
        if (item == b' ') { // if iterator is a byte space literal
            return i; // return the size in bytes until then
        }
    }
    passed_string.len() // if nothing found return the full amount of bytes from passed_string
}