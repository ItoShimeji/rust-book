use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Rust の基本として、ここでは &(*self).0 が行われいている
        &self.0
    }
}

fn main() {
    {
        let x = 5;
        let y = &x;

        assert_eq!(5, x);
        assert_eq!(5, *y);
        // assert_eq!(5, y); {integer} != &{integer} ということ
    }

    {
        let x = 5;
        let y = Box::new(x);

        assert_eq!(5, x);

        // &i32 の時と同じように * (dereference operator) で値を取り出すことができる
        assert_eq!(5, *y);
    }

    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);

        // ここでは *(y.deref()) が暗黙的に行われている
        // *y の呼び出し方法によっては Copy や Clone が必要になる（= deref method は参照を返している）がこのマクロはどちらも必要ない。
        assert_eq!(5, *y);
    }

    {
        let m = MyBox::new(String::from("Rust"));

        // ここで &m はそのままだと &MyBox<String> である
        // ただ、関数呼び出しは deref cite であり、暗黙的な複数の deref が許される
        // &MyBox<String> -> &String -> &str と2回の deref を行い、関数の引数に沿った形に変換される
        // deref coercion では &T -> &U の変換を暗黙的に行うため、以下は hello(m) ではダメ
        // 参考) https://doc.rust-lang.org/book/ch15-02-deref.html#handling-deref-coercion-with-mutable-references
        hello(&m);
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}
