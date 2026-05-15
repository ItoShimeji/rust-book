pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // 外のモジュールで定義したものを全て使えるようにする。
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    // 失敗テスト
    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // 否定を検証
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

// #[derive(Debug, PartialEq)]
// 構造体に以上のように定義すると、debug 表示と、比較ができるようになる。
// Debug は js でオブジェクトを log にそのまま出せるようにする機能を後から実装する感じ
// PartialEq は値の比較となり、普通の言語が参照の比較をするところ、少し特殊。
// Rust は基本的に参照同士の比較を行うことがない（所有している変数が常に1つである）ため、値での比較となるのかな？
// 当然コストは高いため、独自で id だけを比較するようにしたりするらしい。
