pub trait Summary {
    fn summarize(&self) -> String {
        String::from("this is a default thing baby...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

fn main() {
    let post = SocialPost {
        username: String::from("Mike"),
        content: String::from("This is the content"),
        reply: false,
        repost: false,
    };

    println!("{}", post.summarize());
}
