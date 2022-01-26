fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    // let article = NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from(
    //         "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
    //     ),
    // };
    // println!("New article available! {}", article.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
//虽然我们不再直接为 NewsArticle 定义 summarize 方法了，但是我们提供了一个默认实现并且
// 指定 NewsArticle 实现 Summary trait。因此，我们仍然可以对 NewsArticle 实例调用
// summarize 方法
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
// impl Summary for NewsArticle {
// fn summarize(&self) -> String {
//     format!("{}, by {} ({})", self.headline, self.author, self.location)
// }
// }
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
