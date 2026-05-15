use traits::{SocialPost, Summary};

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    let post2 = returns_summarizable();

    println!("1 new post: {}", post.summarize());
    println!("1 new post: {}", post2.summarize());
}

// trait 実装を持つ構造体を返す関数
pub fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "This function’s signature is less cluttered: The function name, parameter list",
        ),
        reply: false,
        repost: false,
    }
}
