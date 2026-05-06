fn main() {
    let s = String::from("Hello, world!");

    {
        let index = first_word(&s);

        println!("first word is separated at {index}th char")
    }

    {
        let hello = &s[0..5];
        let world = &s[6..12];

        println!("s1: {hello}, s: {world}");
    }

    {
        // 引数の型が &str のため、slice に変換して渡す
        let first = first_word_with_slice(&s[..]);
        println!("first word is '{first}'");
    }

    {
        // s は slice 型！
        let s: &str = "Hello, world!";
    }
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// String 型は必要なく、この関数が必要なのは文字列の中身を読む能力
// そのため、&Stringを受け取らない
fn first_word_with_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
