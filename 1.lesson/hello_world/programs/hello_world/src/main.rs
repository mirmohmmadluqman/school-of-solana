pub struct NewsArticle {
    pub author: String,
    // pub headline: String,
    pub content: String,
}

pub Struct Tweet{
    pub username: String,
    // pub headline: String,
    pub content: String,
    pub reply: bool,
    pub retweer: bool,
}

pub trait aTrait {
    fn aFunction(&self) -> string{
        String::from("abcdefghijklmnopkrstwxyz0123456789 ---Default-Implementation/String(Return---")
    }
}

impl aTrait for NewsArticle {
    fn aFunctions(&self) -> string {
        format!("Headline: {} || Content: {}", self.headline, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username String::from("@mirmohmadluqman")
        // headline: String::from("Hello World"),
        content: String::from("This is my first tweet!"),
        reply: false,
        retweer: false,
    }

    let article = NewsArticle {
        autor = String::from("Github Copilot"),
        // headline: String::from("Actually, currently, I am shit"), // Suggested by Github Copilot!?
        content: String::from("But I will be better soon!"), // Suggested by Github Copilot!?
        
}