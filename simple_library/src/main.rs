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
enum ReturnValues<'a> {
    AddOk(String),
    AddErr(String),

    RemoveOk(String),
    RemoveErr(String),

    SearchOk(String, & 'a Book),
    SearchErr(String),

    CheckoutOk(String),
    CheckoutErr(String),

    ReturnBookOk(String),
    ReturnBookErr(String),
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

    fn search(&mut self, title: String) -> ReturnValues {
        match self.books.get(&title) {
            Some(book) => {
                ReturnValues::SearchOk("Book found!".to_string(), book)
            }

            None => ReturnValues::SearchErr("Book not found".to_string())
        }
    }

    fn checkout(&mut self, title: String) -> ReturnValues {
        match self.books.entry(title) {
            Entry::Occupied(entry) => {
                let book = entry.into_mut();

                if book.available == true {
                    book.available = false;

                    return ReturnValues::CheckoutOk("Book as been checked out to user!".to_string())
                }

                ReturnValues::CheckoutErr("Book has already been checked out to another user".to_string())
            }

            Entry::Vacant(_) => {
                ReturnValues::CheckoutErr("Book not found".to_string())
            }
        }
    }

    fn return_book(&mut self, title: String) -> ReturnValues {
        match self.books.entry(title) {
            Entry::Occupied(entry) => {
                let book = entry.into_mut();

                if book.available == false {
                    book.available = true;

                    return ReturnValues::ReturnBookOk("Book has been returned".to_string())
                }

                ReturnValues::ReturnBookErr("Book has already been returned".to_string())
            }

            Entry::Vacant(_) => ReturnValues::ReturnBookErr("Book not found".to_string())
        }
    }

    fn list(&self) {
        if self.books.len() > 0 {
            println!("List of books: ");

            for (_, book) in &self.books {
                println!("Title: {}", book.title);
                println!("Author: {}", book.author);

                if book.available == true {
                    println!("Availability: Yes");
                } else {
                    println!("Availability: No");
                }

                println!();
            }
        } else {
            println!("No books in library");
            println!();
        }
    }
}

struct User {
    name: String,
    id: u32,
    checked_books: Vec<Book>,
}

fn main() {
    let mut library = Library::new_library();

    let mut result = library.add(String::from("Intel For Idiots"), String::from("Ivan"));
    println!();
    match result {
        ReturnValues::AddOk(message) => println!("{message}"),

        (_) => {}
    }

    println!("Books in library: {}", library.books.len());

    for (key, value) in &library.books {
        println!("Title: {key}, Author: {}, Availability: {}", value.author, value.available);
    }

    result = library.remove(String::from("Intel Fr Idiots"));
    println!();
    match result {
        ReturnValues::RemoveOk(message) => println!("{message}"),
        ReturnValues::RemoveErr(message) => println!("{message}"),
        (_) => {}
    }

    println!("Books in library: {}", library.books.len());

    for (key, value) in &library.books {
        println!("Title: {key}, Author: {}, Availability: {}", value.author, value.available);
    }

    result = library.search(String::from("Intel For Idiots"));
    println!();
    match result {
        ReturnValues::SearchOk(message, book) => {
            println!("{message}");

            println!("Book details");
            println!("Author: {}", book.author);
            println!("Title: {}", book.title);
            println!("Availability: {}", book.available);
            println!();
        }

        ReturnValues::SearchErr(message) => println!("{message}"),

        (_) => {}
    }

    result = library.checkout(String::from("Intel For Idiots"));
    println!();

    match result {
        ReturnValues::CheckoutOk(message) => println!("{message}"),
        ReturnValues::CheckoutErr(message) => println!("{message}"),

        (_) => {}
    }
    println!();

    for (key, value) in &library.books {
        println!("Title: {key}, Author: {}, Availability: {}", value.author, value.available);
    }

    result = library.checkout(String::from("Intel For Idits"));
    println!();

    match result {
        ReturnValues::CheckoutOk(message) => println!("{message}"),
        ReturnValues::CheckoutErr(message) => println!("{message}"),

        (_) => {}
    }
    println!();

    result = library.return_book(String::from("Intel For Idiots"));

    match result {
        ReturnValues::ReturnBookOk(message) => println!("{message}"),
        ReturnValues::ReturnBookErr(message) => println!("{message}"),

        (_) => {}
    }

    result = library.return_book(String::from("Intel For Idiots"));

    match result {
        ReturnValues::ReturnBookOk(message) => println!("{message}"),
        ReturnValues::ReturnBookErr(message) => println!("{message}"),

        (_) => {}
    }

    library.add("Did ya know?".to_string(), "Three Sinners".to_string());
    library.add("Running away".to_string(), "Three Sacred Souls".to_string());
    library.add("Changes".to_string(), "Charles Bradley".to_string());

    
    library.checkout("Did ya know?".to_string());

    library.list();
}