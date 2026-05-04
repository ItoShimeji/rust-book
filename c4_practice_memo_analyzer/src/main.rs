fn main() {
    let mut memo = String::from("rust ownership borrowing slices");

    // Option を学習していないため、末端の空白や空白の連続は無視
    let first = first_word(&memo);
    let last = last_word(&memo);
    let count = count_words(&memo);

    println!("memo: {memo}");
    println!("first: {first}");
    println!("last: {last}");
    println!("count: {count}");

    // {
    //     let first = first_word(&memo);
    //     first の 参照が残っている状態で mutation できない
    //     clear_memo(&mut memo);
    //     println!("{first}");
    // }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn last_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let len = s.len();

    for (i, &item) in bytes.iter().rev().enumerate() {
        if item == b' ' {
            return &s[len - i..];
        }
    }

    &s[..]
}

// コメントアウトは空白の連続や末端空白に対応する実装だが、
// usize <-> isize の比較の安全な方法がわからず断念
// space_count を ts の number | null みたいに最初定義するべきか？
fn count_words(s: &str) -> usize {
    let bytes = s.as_bytes();
    let mut space_count = 1;
    // let mut last_space_index = -1;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // if i != last_space_index + 1 {
            //     space_count += space_count;
            // }
            space_count += 1;
            // last_space_index = i;
        }
    }

    space_count
}

fn clear_memo(s: &mut String) {
    s.clear();
}
