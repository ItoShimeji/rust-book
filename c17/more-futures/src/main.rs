use std::{thread, time::Duration};

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;

        // await が宣言されているところで初めて runtime に処理が移るため、
        // 同期的な部分はまとめて実行される
        // 'a' started.
        // 'a' ran for 30ms
        // 'a' ran for 10ms
        // 'a' ran for 20ms
        // 'b' started.
        // 'b' ran for 75ms
        // 'b' ran for 10ms
        // 'b' ran for 15ms
        // 'b' ran for 350ms
        // 'a' finished.
    });

    trpl::block_on(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;

        // await で処理が交換される
        // 'a' started.
        // 'a' ran for 30ms
        // 'b' started.
        // 'b' ran for 75ms
        // 'a' ran for 10ms
        // 'b' ran for 10ms
        // 'a' ran for 20ms
        // 'b' ran for 15ms
        // 'a' finished.
    });

    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;

        // yield_now() で runtime に処理を返せる
        // 応答性が必要な処理(サーバー)などで処理を分割して行うのに使用することもある
        // 'a' started.
        // 'a' ran for 30ms
        // 'b' started.
        // 'b' ran for 75ms
        // 'a' ran for 10ms
        // 'b' ran for 10ms
        // 'a' ran for 20ms
        // 'b' ran for 15ms
        // 'a' finished.
    });

    // 今回は sleep が同期的だが、仮に非同期にした場合は合計時間が短い future が先に終了する直感的なものになる
    // runtime 側が sleep 終了した future を poll するからである
}
