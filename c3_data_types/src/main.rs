use std::io;

fn main() {
    // tuple
    // {
    //     let tup: (i32, i8, f64) = (23, 4, 56.8);

    //     let (x, y, _) = tup;

    //     println!("x: {x}, y: {y}, z: {}", tup.2);

    //     // 空を表す値
    //     // let unit = ();
    // }

    // array
    // {
    //     let a: [i32; 5] = [1, 2, 3, 4, 5];

    //     let first = a[0];
    //     let last = a[a.len() - 1];

    //     println!("first: {first}, last: {last}");
    // }

    // array invalid access
    {
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
    }
}
