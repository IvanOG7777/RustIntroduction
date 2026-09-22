use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Book {
    title: String,
    author: String,
    available: bool,
}

struct User {
    name: String,
    id: u32,
    checked_books: Vec<Book>,
}


// Instance of library will own book when inserted
struct Library {
    // use book title as key, and book as value
    books: HashMap<String, Book>,
    users: HashMap<u32, User>
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

    DeleteUserOk(String),
    DeleteUserErr(String),


}

impl Library {

    // pass ID for now, might make random one later
    fn new_user(name: String, id: u32) -> User {
        User {
            name,
            id,
            checked_books: Vec::new(),
        }
    }

    fn delete_user(&mut self, id: u32) -> ReturnValues {
        match self.users.get(&id) {
            Some(user) => {
                if user.checked_books.len() > 0 {
                    return ReturnValues::DeleteUserErr("User still has books checked out".to_string())
                }

                let user_name = user.name.clone();
                self.users.remove(&id);
                ReturnValues::DeleteUserOk(format!("User: {user_name}, has been deleted"))
            }

            None => ReturnValues::DeleteUserErr("User not found".to_string())
        }
    }

    // new library function to create a new hashmap for instance of library
    fn new_library() -> Library {
        Library {
            books: HashMap::new(),
            users: HashMap::new(),
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

    fn checkout(&mut self, title: String, user_id: u32) -> ReturnValues {
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

fn main() {
    let mut library = Library::new_library();
}