use std::fmt::format;

// Define the trait
trait Summary {
    // this like class
    fn summarize(&self) -> String; // this like abstract method
}

// Implement the trait for a struct
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// like class NewsArticle extends Summary
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({}): {}",
            self.headline, self.author, self.location, self.content
        )
    }
}

// Implement the trait for another struct
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {} (reply: {}, retweet: {})",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Penguins win the ice hockey championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best ice hockey team in the NHL.",
        ),
    };

    // create instance of Tweets
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("people are really good at using computers"),
        reply: false,
        retweet: false,
    };

    println!("News article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
}
