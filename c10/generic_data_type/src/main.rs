struct Point<T> {
    x: T,
    y: T,
}

// impl も generics が使える
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 特定の型に絞ったメソッドを定義できる
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {result}");
    }

    {
        let wont_work = Point { x: 5, y: 4 };
    }
}

// 不等号による比較を行うためには、std::cmp::PartialOrd trait を実装していないといけない。
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}
