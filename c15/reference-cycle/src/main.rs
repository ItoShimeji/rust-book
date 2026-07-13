use std::cell::RefCell;
use std::rc::{Rc, Weak};

// RefCell を使用しているのは、Rc<Node> からは共有参照しか取得できず、この状態で mutation する方法が基本的にないからである
// RefCell により参照から mutation を可能にする
// 以下のように Rc によって Node を leaf と branch の 2 つから所有されるかたちにできる
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// Rust では以下のように C におけるポインタを所有やライフタイムなどの概念から分解していると理解すると良いかも
// Cの Node*
//     ├─ 所有ポインタかもしれない
//     ├─ 非所有ポインタかもしれない
//     ├─ 共有されているかもしれない
//     ├─ 書き換え可能かもしれない
//     └─ danglingかもしれない

// Rust
//     ├─ Box<Node>       単独所有
//     ├─ Rc<Node>        共有所有
//     ├─ Weak<Node>      非所有参照
//     ├─ &Node           一時的な共有借用
//     ├─ &mut Node       一時的な排他借用
//     └─ RefCell<Node>   実行時検査による内部可変性

fn main() {
    {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        // 論理的には循環参照が起きているが、children -> parent が weak のため、スタックオーバーフローが防がれる
        // leaf parent = Some(Node { value: 5, parent: RefCell { value: (Weak) },
        // children: RefCell { value: [Node { value: 3, parent: RefCell { value: (Weak) },
        // children: RefCell { value: [] } }] } })
    }

    {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });

            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
