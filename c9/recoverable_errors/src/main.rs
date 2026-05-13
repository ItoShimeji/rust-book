use std::fs;
use std::io::{self, Read};
use std::{fs::File, io::ErrorKind};

fn main() {
    {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file_result = match greeting_file_result {
            // Option のように、 Ok, Err は prelude に入っている。
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:?}"),
                },
                _ => {
                    panic!("Problem opeaning the file: {error:?}")
                }
            },
        };
    }

    {
        // unwrap は Ok の中身を返すか、panic するか。
        // expect でメッセージを設定できる。
        // let greeting_file = File::open("hello2.txt").unwrap();
    }

    {
        // let username = read_username_from_file().unwrap();
        // let username = read_username_from_file2().unwrap();
        let username = read_username_from_file3().unwrap();
        eprintln!("username is {username}");
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello3.txt");

    // ? 演算子によって、Result::Ok は中身が返され、Err は return される。
    // 関数内で Result を呼び出し元に伝播させるのに重宝しそう。
    // 当然、この関数の返す値と Err の型が合っていないといけない。
    let mut username_file = username_file_result?;

    let mut username = String::new();

    // Err の場合は ? で返され、Ok の場合は展開されたのを最終行で Ok に再度詰めている。
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();

    // ? を使うことで、Result を返す関数・メソッドのチェーンができる。
    File::open("hello3.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// 上の2つをラップしたような便利な関数もある。
fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello3.txt")
}
