use std::thread;
use std::time::Duration;

fn main() {
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the main thread!");
            thread::sleep(Duration::from_millis(1));
        }

        // ここで handle を生成した thread の実行終了を待機
        handle.join().unwrap();
    }

    {
        let v = vec![1, 2, 3];

        // thread のクロージャで外部環境に存在する値を使用するときは move で所有権を移動させる
        // 移動させないと借用となり、main での変更や drop と不整合が生じる可能性がある
        // このように所有権システムにより複数スレッドの実行が安全となっている
        let handle = thread::spawn(move || {
            println!("Here's a vector: {v:?}");
        });

        handle.join().unwrap();
    }
}
