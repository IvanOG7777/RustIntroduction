struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}
pub trait Summary {
    fn summarize_author(&self)-> String; // define this in implementation

    fn summarize(&self) -> String {
       format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

//
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("Calling notify");
    notify(&article);
}