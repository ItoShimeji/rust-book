// front_of_house は eat_at_restaurant から使う分には同じモジュールだから
// pub にする必要はない。
mod front_of_house {
    // 子以下のモジュールは pub をつける
    pub mod hosting {
        pub fn add_to_whitelist() {}
    }
}

// front_of_house と sibling
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_whitelist();

    // Relative path
    front_of_house::hosting::add_to_whitelist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toasty please", meal.toast);
}

mod back_of_house {
    // seasonal_fruit を外部公開しないのは、OOP のカプセル化と類似する。
    // こちらはモジュール単位のカプセル化であることに注意。
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // 外部から見えなくても、値としては常に存在するため、
        // seasonal_fruit も含めた構造体を返す
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
