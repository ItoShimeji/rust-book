pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
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

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// ある trait を実装した構造体を受ける
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 上の関数定義は下の generics を用いた実装の syntax sugar
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// 2 つの引数が同じ trait を実装していることを宣言する場合は、generics が必要
// pub fn notify<T: Summary>(item1: &T, item2: &T) {}

// 2 つの trait を実装していることは + で表現;
// pub fn notify(item: &(impl Summary + Display)) {}

// where 句で関数宣言の視認性を上げることができる
// fn example<T, U>(t: T, u: U)
// where
//     T: Display + Clone,
//     U: Debug + PartialEq,
// {
//     // 関数の中身
// }

// https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
// については、以下のように標準ライブラリで定義されていることで実装者は Display trait の実装さえ行えば、
// to_string() が使用できるようになる。trait による抽象化のメリット。
// impl<T: Display> ToString for T {
// --snip--
// }
