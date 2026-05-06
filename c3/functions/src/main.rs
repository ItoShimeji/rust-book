fn main() {
    println!("Hello, world!");

    only_print(five());
}

fn only_print(x: i32) {
    println!("'x is {x}' from another function.");
}

fn five() -> i32 {
    5
}
