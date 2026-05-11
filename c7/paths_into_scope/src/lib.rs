mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// ここで hosting を名前空間に追加
// 使用する関数を直接定義しないのは、使用側でモジュールを明確に意識できるようにするため。
// structs, enums などはフルパスを指定する。
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

mod customer {
    // use は単なる symbolic link　で、モジュールシステムのルールを変えないため、
    // モジュールを切った場合、その中で use を書く。
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
