fn main() {
    let condition = false;

    let number = if condition { 5 } else { 6 };

    println!("the number is {number}");

    loop_ten_times();

    loop_an_array();

    loop_with_range()
}

fn loop_ten_times() {
    let mut index = 0;

    loop {
        if index >= 10 {
            break;
        } else {
            println!("loop count: {index}");

            index += 1;
            continue;
        }
    }
}

fn loop_an_array() {
    let a = [1, 2, 3, 4, 5];

    for e in a {
        println!("element is {e}");
    }
}

fn loop_with_range() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
