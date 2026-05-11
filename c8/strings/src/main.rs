fn main() {
    {
        let data = "initial contents";

        // 新しく heap 上に値をコピーして返す
        // String::from("initial contents") と同じ。
        let s = data.to_string();
    }

    {
        let mut s = String::from("foo");
        s.push_str("bar");
    }

    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        // 演算子オーバーロードを行なっている。
        // n add(self, rhs: &str) -> String ということ。
        // s1 に s2 のコピーを追加しているようなもので、メモリ効率が良い。
        let s3 = s1 + &s2;

        println!("s3 is '{s3}'");
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // format! は所有権を受け取らないため、上の加算よりもメモリ効率が悪そう。
        let s = format!("{s1}-{s2}-{s3}");
    }

    {
        // マルチバイト文字を含む UTF-8 で符号化されるため、 &hello[0] のようなインデックスによる
        //  O(1) の文字の取得はできない。
        // そのため、Rust ではインデックスではなく、以下のような API を提供する。

        for c in "モモンガ学入門".chars() {
            println!("{c}");
        }

        for b in "モモンガ学入門".bytes() {
            println!("{b}");
        }
    }
}
