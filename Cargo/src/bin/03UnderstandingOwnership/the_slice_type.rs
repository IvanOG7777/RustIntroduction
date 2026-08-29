use std::path::Component::ParentDir;

fn main() {

    let mut string = String::from("HelloWorld from");
    let first_word = find_first_word(&string); // should return "HelloWorld";

    println!("First word within string is {first_word}");

    string = String::from("Hello world");

    let word = find_first(&string); // will get the value 5 (index of where first space was seen);
    string.clear(); // clears string to "";

    println!("First index where ' ' was seen is at: {word}");

    {
        let s = String::from("hello world");

        let hello = &s[..5]; // variable hello gets a borrow of s from index 0 to 5;
        let world = &s[6..11]; // variable hello gets a borrow of s from index 6 to 11;

        println!("{hello} {world}");

        let s2 = String::from("hello");

        // both are equivalent
        let with_front_zero = &s2[0..2];
        let without_front_zero = &s2[..2];

        println!("{with_front_zero} {without_front_zero}");

        let length = s.len();
        let with_length = &s[3..length];
        let without_back_zero = &s[3..];

        println!("{with_length}, {without_back_zero}");

        // full copy
        let both_variables = &s[0..length];
        let only_dots = &s[..];

        println!("{both_variables}, {only_dots}");
    }

    // new find_first function
    {
        let s = String::from("Hello world");
        let word = find_first_new(s);

        println!("The first word is: {word}");
    }
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
// Pass stack allocated reference to a String Object
fn find_first(passed_string:  &String) -> usize { // Returns the byte index of where the first space was seen
    let bytes = passed_string.as_bytes(); // Gives borrow of bytes in string

    // i is used as the index while item is the current character at that index
    for (i, &character) in bytes.iter().enumerate() { // iterate though bytes and get their index
        if (character == b' ') { // if iterator is a byte space literal
            return i; // return the index of where first space was seen
        }
    }
    passed_string.len() // if nothing found return the full amount of bytes from passed_string
}

fn find_first_new(passed_string: String) -> String {
    let bytes = passed_string.as_bytes();

    for (i, &character) in bytes.iter().enumerate() {
        if character == b' ' {
            let new_string = &passed_string[0..i];

            let first_word = String::from(new_string);
            return first_word;
        }
    }

    passed_string
}