#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &U {
        &self.y
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("速報: {}", item.summarize());
}

fn main() {
    // ===========================
    // 1. ジェネリクス
    // ===========================
    println!("\n--- 1. ジェネリクス ---");
    let p1 = Point { x: 5, y: 10 };
    println!("{:?}", p1);
    println!("{:?}", p1.get_x());
    println!("{:?}", p1.get_y());

    let p2 = Point {
        x: 10.5,
        y: "Hello",
    };
    println!("{:?}", p2);
    println!("{:?}", p2.get_x());
    println!("{:?}", p2.get_y());

    // ===========================
    // 2. トレイトとトレイト境界
    // ===========================
    println!("\n--- 2. トレイトとトレイト境界 ---");
    let news_article = NewsArticle {
        headline: String::from("最新ニュースです"),
        author: String::from("田中"),
    };
    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
    };

    notify(&news_article);
    notify(&tweet);
}
