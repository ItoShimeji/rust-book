use std::thread;

fn main() {
    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        let mut borrows_mutably = || list.push(7);

        borrows_mutably();
        // 以下は closure を実行したあとではないと呼べない
        // closure で mutable borrow をしているところに追加で immutable borrow をするのは禁止
        println!("After calling closure: {list:?}");
    }

    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {list:?}");

        // 別スレッドに渡す closure では、たとえ中で読むだけでも、
        // 外側の変数を借用ではなく所有権ごと渡す必要がある場合がある
        // そのため、以下のように move で所有権の移動を行わせることができる
        // 後の並列実行のチャプターで学ぶ
        thread::spawn(move || println!("From thread: {list:?}"))
            .join()
            .unwrap();
    }

    {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        let mut num_sort_operations = 0;
        // ここでの closure は FnMut のため、このように外部環境を mutation できる
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{list:#?}, sorted in {num_sort_operations} operations");
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
