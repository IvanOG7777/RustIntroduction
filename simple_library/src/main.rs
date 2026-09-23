use std::any::Any;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Book {
    title: String,
    author: String,
    available: bool,
    current_owner: Option<u32>,
}

struct User {
    name: String,
    id: u32,
    checked_books: HashMap<String, Book>,
}

impl User {
    // pass ID for now, might make random one later
    fn new_user(name: String, id: u32) -> User {
        User {
            name,
            id,
            checked_books: HashMap::new(),
        }
    }

    fn list(&self) {

        if (self.checked_books.len() > 0) {
            println!("Checked out books");
            for (_, book) in &self.checked_books {
                println!("Title: {}", book.title);
                println!("Author: {}", book.author);
                println!("Availability: {}", book.available);
            }
        }

        println!("No checked out books");
    }

    fn add(&mut self, title: String, author: String) -> ReturnValues {
        let book = Book::new_book(title, author, true);

        let title = book.title.clone();
        let author_ = book.author.clone();

        self.checked_books.insert(title.clone(), book);

        ReturnValues::AddOk(format!("Book: {}, has been added to {}, checkout books!", title, self.name))
    }
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
            available,
            current_owner: None
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

    FindUserOk(String, & 'a mut User),
    FindUserErr(String),
}

impl Library {

    fn add_user(&mut self, user: User) -> ReturnValues {
        let user_name = user.name.clone();
        self.users.insert(user.id, user);
        ReturnValues::AddOk(format!("User: {} has been added", user_name))
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

    fn find_user(&mut self, user_id: u32) -> ReturnValues {
        match self.users.get_mut(&user_id) {
            Some(user) => {
                let user_name = user.name.clone();
                ReturnValues::FindUserOk(format!("User: {user_name}, found"), user)
            }

            None => {
                ReturnValues::FindUserErr("User not found".to_string())
            }
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

        let book = match self.books.get_mut(&title) {
            Some(book) => book,
            None => {return ReturnValues::CheckoutErr("Book not found".to_string())}
        };

        let user = match self.users.get_mut(&user_id) {
            Some(user) => user,
            None => {return ReturnValues::CheckoutErr("User not found".to_string())}
        };

        if book.available == true {
            let copy_title = book.title.clone();
            let author_copy = book.author.clone();

            user.add(copy_title, author_copy);

            book.available = false;
            book.current_owner = Some(user.id);

            return ReturnValues::CheckoutOk(format!("Book: {}, has been checked out by {}", book.title, user.name))
        }

        ReturnValues::CheckoutErr(format!("Book: {}, has already been checked", book.title))
    }

    fn return_book(&mut self, title: String) -> ReturnValues {
        let title_copy = title.clone();
        match self.books.entry(title) {
            Entry::Occupied(entry) => {
                let book = entry.into_mut();

                let owner_id = match book.current_owner {
                    Some(id) => id,
                    None => {return ReturnValues::ReturnBookErr("Book has already been returned".to_string())}
                };


                if book.available == false {
                    let user = match self.users.get_mut(&owner_id) {
                        Some(user) => user,
                        None => {return ReturnValues::ReturnBookErr("User not found".to_string())}
                    };

                    user.checked_books.remove(&title_copy);
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

    let user1 = User::new_user("Ivan".to_string(), 1);
    let user2 = User::new_user("Zakai".to_string(), 2);
    let user3 = User::new_user("Cassandra".to_string(), 3);
    let user4 = User::new_user("Louis".to_string(), 4);

    let mut users:Vec<User> = Vec::new();

    users.push(user1);
    users.push(user2);
    users.push(user3);
    users.push(user4);

    for user in users {
        let result = library.add_user(user);

        match result {
            ReturnValues::AddOk(message) => println!("{}", message),
            _ => {}
        }
    }
    println!();

    library.add("Intel for Idiots".to_string(), "The Drink".to_string());
    library.add("My brother just another me".to_string(), "Lucci".to_string());
    library.add("Lobat".to_string(), "Mazda Nava".to_string());

    library.list();

    let mut result = library.checkout("Intel for Idiots".to_string(), 1);

    match result {
        ReturnValues::CheckoutOk(message) => println!("{}", message),
        ReturnValues::CheckoutErr(err) => println!("{err}"),
        ReturnValues::FindUserErr(err) => println!("{err}"),
        (_) => {},
    }
    println!();

    library.list();

    println!();

    for (_, user) in &library.users {
        user.list();
    }

    result = library.return_book("Intel for Idiots".to_string());

    match result {
        ReturnValues::ReturnBookOk(message) => println!("{}", message),
        ReturnValues::ReturnBookErr(err) => println!("{err}"),
        (_) => {}
    }

    library.list();
}