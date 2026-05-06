fn main() {
    let n: u32 = 30;
    let result = fibonacci(n);

    println!("{n}th fibonacci number is {result}")
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 2) + fibonacci(n - 1),
    }
}
