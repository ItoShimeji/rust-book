use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    // drop method 内で sender の drop 後の状態を表現するため Option
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();

        // Rc の スレッド安全版である Arc を持ちいてそれぞれの worker に receiver の所有権を渡す
        // channel を処理の queue に使うため、一つの channel の receiver をそれぞれの配布する
        // スレッド間で安全にロックを取るために Mutex を使用
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        // worker を借用すると、thread を消費する join method を呼べない
        // そのため、Vec から要素を抜き取ることで所有権を得ている
        // drain が特別なわけではなく、pop などで要素を一個ずつ抜き取っても良い
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            // panic 後の clean up 中に呼ばれるコードで panic をするのはプロダクションコードでは好ましくない
            worker.thread.join().unwrap();
        }
    }
}

// `+` で型の論理積をとっている
// `dyn`は条件を trait object に変換するような働き
// クロージャをサイズの確定した一つの型として扱うため Job 型を導入
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv();

                // ThreadPool の drop method 内で sender が drop されると、channel が閉じ、
                // Err のメッセージが届く
                match message {
                    Ok(job) => {
                        println!("Worker {} got a job; executing.", id);

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker { id, thread }
    }
}
