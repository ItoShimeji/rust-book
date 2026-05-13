use std::collections::HashMap;

fn main() {
    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 10);

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }

    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map: HashMap<String, &str> = HashMap::new();
        map.insert(field_name, &field_value);

        // map に String などの参照型を渡すと、所有権は map に移る。
        // 文字列として渡したい場合は field_value のように &str を渡す。
        // println!("{field_name}: {field_value}");
    }

    {
        // 英語表現: vice versa 逆もまた然り

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        // ここでは上書きが行われる。
        scores.insert(String::from("Blue"), 25);

        println!("{scores:?}")
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        // entry() は hash map に対する 可変借用を含む Entry enum を返す。
        // or_insert() は Entry enum を消費して value をセットし、
        // その value への可変借用を返す。（以下の場合だと戻り値を変数に入れていないため、関係ない。）
        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{scores:?}")
    }

    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            // value への 可変参照を受け取っているため、変更ができる。
            *count += 1;
        } // ここで count は消える

        println!("{map:?}");
    }
}
