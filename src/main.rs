pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(read more...)")
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
}

pub struct NewComment {
    pub author: String,
}

impl Summary for NewComment {}

// 使用 trait 作为参数，参数对象必须有 trait 的实现
// 我们可以传递任何 NewsArticle 或 Tweet 的实例来调用 notify。
// 任何用其它如 String 或 i32 的类型调用该函数的代码都不能编译，因为它们没有实现 Summary。

pub fn notify(item: impl Summary) {
    println!("Breaking news {}",item.summarize());
}


// 返回一个 trait 不知道能干啥。。。
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("hello world"),
        location: String::from("beijing"),
        author: String::from("matianqi"),
        content: String::from("read the book")
    };

    let comment = NewComment {
        author: String::from("matianqi")
    };
    
    println!("1 new tweet: {}", tweet.summarize());

    println!("1 new article: {}",article.summarize());

    println!("1 new comment: {}", comment.summarize());
    
    notify(comment);

    notify(article);

    notify(tweet);




}