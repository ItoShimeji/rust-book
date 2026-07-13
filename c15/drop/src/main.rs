struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}", self.data);
    }
}

fn main() {
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created");

        // output:
        // CustomSmartPointers created
        // Dropping CustomSmartPointer with data other stuff
        // Dropping CustomSmartPointer with data my stuff
        //
        // LIFO 的に drop が呼ばれていく
        // これは Go の defer と同じかな
    }

    {
        // 以下のようにすると、早期に drop できる
        // drop fn は所有権を受け取るため、それ以降でもう一度 free されるのを防ぐ
        // drop method が所有権を受け取らずに &mut self を受け取るのは、free の前の hook という立ち位置だから
        let c = CustomSmartPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main");
    }
}
