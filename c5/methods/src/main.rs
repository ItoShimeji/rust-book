#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// メソッド定義
impl Rectangle {
    // &self: &Self の省略
    // 中で mutation したい場合は &mut self で可変参照を受ける。
    // self で所有権を受け取ることもできるが、method で値が消費されてしまう。
    // js では、変数を immutable に扱うことで安全なプログラムを実現するパターンが推奨されるが、
    // Rust では ownership のおかげで危険な mutation を禁止できる。
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // self を使わない実装は Associated Functions と呼ばれる
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            // この呼び出しは (&rect1).area() と同じことである。
            // method の型定義から暗黙的に参照をとっている。
            rect1.area()
        );
    }

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    {
        // Associated Functions は :: で呼び出す
        let square = Rectangle::square(4);

        println!(
            "aquare's width is {}, it's height is {}",
            square.width, square.height
        )
    }
}
