struct User {
    username: String,
    age: u32,
}

#[derive(Copy, Clone)]
struct Stats {
    building: u32,
    room: u32,
    active: bool,
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

    // user1.print_user(); // error
    user2.print_user();

    let user3 = User::create_user(String::from("Ivan"), 22);
    let user4 = &user3;

    user3.print_user();
    user4.print_user();

    let stats1 = Stats {
        building: 1,
        room: 202,
        active: true
    };

    let stats2 = stats1;

    println!("Room: {}", stats1.room);
    println!("Room: {}", stats2.room);

    println!()
}