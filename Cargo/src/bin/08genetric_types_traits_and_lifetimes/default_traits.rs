struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct SocialPost {
    username: String,
    content: String,
    reply: bool,
    repost: bool
}
pub trait Summary {

    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_text(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
     format!("@{}", self.author)
    }
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
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

    println!("New article available! {}", article.summarize_text());

    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    };

    println!("1 new post: {}", post.summarize_text());
    
}