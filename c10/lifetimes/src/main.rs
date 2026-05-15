// 構造体の内部に参照を持つことができるが、その場合には lifetime parameter を定義する。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    {
        // 初期値なしで定義すると、後続の代入で型が決定する。
        // 初期値なしの状態ではランタイムの値は存在せず（not null）
        // この状態のまま使用すると、compile error となる。
        // let r;

        {
            // let x = 5;
            // ここでは、main 関数の lifetime よりも短いブロックスコープの x への参照を行っているため、
            // lifetime エラー。
            // r = &x;
        }

        // println!("r: {r}")
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {result}");
    }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();

        // i の lifetime は novel の lifetime となる。
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    {
        // 'static は lifetime がプログラム実行中全てである。
        // 文字列リテラルは所有者がおらず、'static となる。
        // グローバル変数や、コンパイル時に値が確定できることを保証するために使う。
        let s: &'static str = "I have a static lifetime.";
    }
}

// lifetime parameter である 'a によって、引数と戻り値の lifetime の契約(関係)を記述している。
// 'a は2つの引数の lifetime の短い方が採用。
// これがないと、呼び出し側で戻り値の lifetime を決定できず、use-after-free を引き起こす。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
