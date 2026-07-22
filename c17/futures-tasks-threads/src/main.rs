use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    // Thread は一つのスレッドを OS 経由で立ち上げ、同期的に一つのプログラムを動かす
    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // future をまとめる Task は複数立ち上げると複数のスレッドに振り分けられ、一つのスレッドに複数割り当てられることもある
    // 一つのスレッドの中ではそれぞれの Task が並行処理される
    // これは goroutine のような扱いやすい軽量な実行単位を
    // Rust では並行処理を実現するための表現として Future を使用するため、
    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
