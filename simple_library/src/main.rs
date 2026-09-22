use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Book {
    title: String,
    author: String,
    available: bool,
}

// Instance of library will own book when inserted
struct Library {
    // use book title as key, and book as value
    books: HashMap<String, Book>,
}

impl Book {
    fn new_book(title: String, author: String, available: bool) -> Book {
        Book {
            title,
            author,
            available
        }
    }
}

// custom enum return values
enum ReturnValues {
    AddOk(String),
    AddErr(String),

    RemoveOk(String),
    RemoveErr(String),

}

impl Library {

    // new library function to create a new hashmap for instance of library
    fn new_library() -> Library {
        Library {
            books: HashMap::new(),
        }
    }

    fn add(&mut self, title: String, author: String) -> ReturnValues {
        let new_book = Book::new_book(title, author.clone(), true);
        let title = new_book.title.clone();
        let author_ = new_book.author.clone();

        self.books.insert(title.clone(), new_book);

        ReturnValues::AddOk(format!("Book: {}, by {} was added", title, author_))
    }

    fn remove(&mut self, title: String) -> ReturnValues {
        match self.books.entry(title) {
            Entry::Occupied(entry) => {
                let book = entry.into_mut();
                let book_title = book.title.clone();
                let book_author = book.author.clone();
                self.books.remove(&book_title);

                ReturnValues::RemoveOk(format!("The book: {} by {}, has been removed", book_title, book_author))
            }

            Entry::Vacant(_) => {
                ReturnValues::RemoveErr("Book not found".to_string())
            }
        }
    }

    fn search() {}

    fn checkout() {}

    fn return_book() {}

    fn list() {}
}

fn main() {
    let mut library = Library::new_library();

    let mut result = library.add(String::from("Intel For Idiots"), String::from("Ivan"));

    match result {
        ReturnValues::AddOk(message) => println!("{message}"),

        (_) => {}
    }

    println!("Books in library: {}", library.books.len());

    for (key, value) in &library.books {
        println!("Title: {key}, Author: {}, Availability: {}", value.author, value.available);
    }

    result = library.remove(String::from("Intel Fr Idiots"));

    match result {
        ReturnValues::RemoveOk(message) => println!("{message}"),
        ReturnValues::RemoveErr(message) => println!("{message}"),
        (_) => {}
    }

    println!("Books in library: {}", library.books.len());

    for (key, value) in &library.books {
        println!("Title: {key}, Author: {}, Availability: {}", value.author, value.available);
    }
}