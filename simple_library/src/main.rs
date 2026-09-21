use std::collections::HashMap;

struct Book {
    title: String,
    author: String,
    available: bool,
}

struct Library {
    books: HashMap<String, Book>,
}

impl Library {

    fn add() {}

    fn remove() {}

    fn search() {}

    fn checkout() {}

    fn return_book() {}

    fn list() {}
}

fn main() {
    println!("Hello, world!");
}