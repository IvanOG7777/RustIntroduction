use std::env;
use std::fs;
fn main() {

    let args: Vec<String> = env::args().collect();

    // let first_value = &args[0];
    let query = &args[1];
    let file_path = &args[2];

    // println!("First arg to vector is: {first_value}");
    println!("Searching for query: {query}");
    println!("In the file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Failed to read from the file");

    println!("With the text:\n {contents}");
}