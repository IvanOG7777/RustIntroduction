use crate::Message::{Text, Image, FriendRequest};
enum Message {
    Text(String, String),
    Image(String, String),
    FriendRequest(String),
}

fn process_message(message_type: Message) {
    match message_type {
        Text(sender, message) => {
            println!("{}: {}", sender, message);
        }
        Image(sender, file_name) => {
            println!("{}: {}", sender, file_name);
        }

        FriendRequest(sender) => {
            println!("{} has sent you a friend request!", sender);
        }
    }
}
fn main() {
    process_message(Text(String::from("Ivan Argueta"), String::from("Hello this is my text to you!")));
    process_message(Image(String::from("Ivan Argueta"), String::from("im_da_goat.png")));
    process_message(FriendRequest(String:: from("John Pork")));
}