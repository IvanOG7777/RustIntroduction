struct Email{
    sender: String,
    subject: String,
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

    // adds new trait to Notification that all extension can see and have the option to use
    fn preview(&self) -> String {
        format!("Message from: {}", self.sender())
    }
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
    // Allows for the preview message to be overridden by the TextMessage struct. Can change the message within
    fn preview(&self) -> String {
       format!("I have overridden the preview() for TextMessage struct")
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
        subject: String::from("Some message"),
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

    println!();
    println!();
    println!("{}", discord.preview());
    send_notification_generic(&discord);

    println!("Overridden message: {}", text.preview());
}
