use std::time::Duration;

fn main() {
    {
        trpl::block_on(async {
            let fut1 = trpl::spawn_task(async {
                for i in 1..10 {
                    println!("hi number {i} from the first task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            let fut2 = trpl::spawn_task(async {
                for i in 1..5 {
                    println!("hi number {i} from the second task!");
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            });

            // 2 つの future はランタイム管理の関数に渡すことで並行処理される
            // 一つの async block の中では直列で実行される
            // これは js の promise と同じ
            trpl::join(fut1, fut2).await;
        })
    }

    {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel();

            // ここで move を宣言することで、tx が借用ではなく、所有権移動となる
            // そのため、ブロック脱出時に tx が drop し、channel が close される
            let tx_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                // channel が close した時にループから脱出する
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            trpl::join(tx_fut, rx_fut).await;
        })
    }

    {
        trpl::block_on(async {
            let (tx, mut rx) = trpl::channel();

            let tx1 = tx.clone();
            let tx1_fut = async move {
                let vals = vec![
                    String::from("hi"),
                    String::from("from"),
                    String::from("the"),
                    String::from("future"),
                ];

                for val in vals {
                    tx1.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(500)).await;
                }
            };

            let rx_fut = async {
                while let Some(value) = rx.recv().await {
                    println!("received '{value}'");
                }
            };

            let tx_fut = async move {
                let vals = vec![
                    String::from("more"),
                    String::from("messages"),
                    String::from("for"),
                    String::from("you"),
                ];

                for val in vals {
                    tx.send(val).unwrap();
                    trpl::sleep(Duration::from_millis(1500)).await;
                }
            };

            // promise.all と同じことを行うマクロ
            trpl::join!(tx1_fut, tx_fut, rx_fut);
        })
    }
}
