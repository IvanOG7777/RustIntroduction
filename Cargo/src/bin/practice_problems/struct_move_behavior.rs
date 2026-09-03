struct User {
    username: String,
    age: u32,
}

impl User {
    fn create_user(username: String, age: u32) -> User {
        User {
            username,
            age
        }
    }

    fn print_user(&self) {
        println!("Name: {}", self.username);
        println!("Age: {}", self.age);
    }
}
fn main() {

    let user1 = User::create_user(String::from("Ivan"), 22);
    let user2 = user1; // user1 loses ownership of User object

    user1.print_user();
    user2.print_user();

    println!()
}