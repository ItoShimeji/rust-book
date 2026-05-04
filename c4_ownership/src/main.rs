fn main() {
    let mut s = String::from("Hello, world!");

    s.push_str(" from rust beginner.");

    let s2 = s;

    println!("{s2}")
}
