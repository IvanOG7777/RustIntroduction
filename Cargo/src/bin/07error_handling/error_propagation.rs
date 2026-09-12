use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn read_username_from_file_operator_shortcut() -> Result<String, io::Error> {

    let mut username_file = File::open("hello.txt")?; // return value should be a File object, else return an Error

    let mut username = String::new();

    username_file.read_to_string(&mut username)?; // if valid read all text from file to username
    Ok(username) // return username
}

// same as above 2 function just a very shorthanded version
fn read_username_from_file_operator_shortcut_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    let username_file = File::open("hello.txt")?.read_to_string(&mut username);
    Ok(username)
}

fn read_username_from_file_operator_shortcut_shorter_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
fn main() {
    // let greeting_file = File::open("hello.txt")?; // can do this since main function cant return a Result(OK, Err)


}