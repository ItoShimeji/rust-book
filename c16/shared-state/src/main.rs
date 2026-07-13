use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    {
        let m = Mutex::new(5);

        {
            // 他の thread が lock をとっていたら失敗する
            let mut num = m.lock().unwrap();
            *num = 6;

            // スコープを抜ける時に num は drop し、lock は解放される
        }

        println!("m = {m:?}");
    }

    {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                // Rc と同じように Arc<Mutex<i32>> -> Mutex<i32> の deref が暗黙的に行われる
                // lock は blocking のため、解放されるまで待機
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
