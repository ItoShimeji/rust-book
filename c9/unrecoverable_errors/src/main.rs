fn main() {
    // panic!("crash and burn");

    // RUST_BACKTRACE=1 cargo run で backtrace が表示される
    let v = vec![1, 2, 3];
    v[99];
}
