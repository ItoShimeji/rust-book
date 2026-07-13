enum List {
    // 通常は構造体の中身はそれぞれ値として所持される
    // 今回は再帰構造となっており、List の heap 上のサイズが決定できないため、
    // heap への pointer である Box を使用する
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    {
        let b = Box::new(2);
        println!("b = {b}");
    }

    {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}
