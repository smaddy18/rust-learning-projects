
trait Summary {
    fn summarize(&self) -> String;
}

struct NewsArticle {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("Article's title: {}, written by @{}", self.title, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize())
}

fn main() {
    let article = NewsArticle {
        title: "Artificial Intelligence".to_string(),
        author: "John".to_string(),
    };

    let tweet = Tweet {
        username: "Mary".to_string(),
        content: "There is a new release of my favorite App".to_string(),
    };

    // println!("{}", article.summarize());
    // println!("{}", tweet.summarize());

    notify(&article);
    notify(&tweet);
}
