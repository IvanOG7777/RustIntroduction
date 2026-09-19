struct Email{
    sender: String,
    body: String,
}

struct TextMessage{
    sender: String,
    body: String
}

struct DiscordMessage{
    sender: String,
    body: String
}

trait Notification {
    fn sender(&self) -> &str;
    fn body(&self) -> &str;
}

impl Notification for Email {
    fn sender(&self) -> &str {
        self.sender.as_str()
    }

    fn body(&self) -> &str {
        self.body.as_str()
    }
}

impl Notification for TextMessage {
    fn sender(&self) -> &str {
        self.sender.as_str()
    }

    fn body(&self) -> &str {
        self.body.as_str()
    }
}

impl Notification for DiscordMessage {
    fn sender(&self) -> &str {
        self.sender.as_str()
    }

    fn body(&self) -> &str {
        self.body.as_str()
    }
}

// pass a reference to an implementation of Notification trait
fn send_notification(item: &impl Notification) {
    // if implemented just call the sender and body functions from Notification
    println!("Notification from {}:", item.sender());
    println!("{}", item.body());
}

// Passed generic type must have Notification implemented
// Just reference the passed type and call the Notification functions
fn send_notification_generic<T: Notification>(item: &T) {
    println!("Notification from {}:", item.sender());
    println!("{}", item.body());
}

fn main() {
    let email = Email {
        sender: String::from("Ivan"),
        body: String::from("I wanna bike"),
    };

    let text = TextMessage {
        sender: String::from("Ivan"),
        body: String::from("Hello you fine asf"),
    };

    let discord = DiscordMessage {
        sender: String::from("Ivan"),
        body: String::from("Hello my kitten"),
    };

    send_notification(&email);
    send_notification(&text);
    send_notification(&discord);

    send_notification_generic(&email);
    send_notification_generic(&text);
    send_notification_generic(&discord);
}
