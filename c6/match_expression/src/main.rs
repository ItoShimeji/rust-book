#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    {
        let coin = Coin::Quarter(UsState::Alabama);
        // 所有権は関数に移動
        value_in_cents(coin);
    }

    {
        // Some() が Option::Some() と書かなくても Option に型推論されるのは、
        // Some が prelude という Rust に定義されているスコープに存在するから
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // arm の順番は Option の場合にどっちが先が良いとかはない（慣習としても）。
        None => None,
        Some(value) => Some(value + 1),
    }
}
