use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    {
        let (tx, rx) = mpsc::channel::<String>();

        // move で tx をクロージャ環境に移動
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();

            // val はすでに消費されているため、使用できない
            // println!("val is {val}");
        });

        // recv() は値の到着を待機する
        // try_recv は non blocking
        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }

    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // 4つの String のメッセージの後に、Sender の drop が通知され、プログラムが終了される
        // rx は内部的に以下のような実装で iter として回せる
        // fn next(&mut self) -> Option<T> {
        //     match self.receiver.recv() {
        //         Ok(value) => Some(value),
        //         Err(_) => None,
        //     }
        // }
        for received in rx {
            println!("Got: {received}");
        }
    }

    {
        let (tx, rx) = mpsc::channel();

        // clone で複数の sender を使用できる
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {received}");
        }
    }
}
