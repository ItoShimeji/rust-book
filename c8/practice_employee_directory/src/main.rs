use std::collections::HashMap;
use std::io;

enum Command {
    Add { name: String, department: String },
    List { department: String },
    Quit,
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();

        let bytes_read = io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // EOF で 0 を返したときに無限ループが発生しないようにする。
        if bytes_read == 0 {
            break;
        }

        let mut parts = input.split_whitespace();

        // ここでコマンドのパースを行う
        let input = match parts.next() {
            Some("Add") => parse_add(parts),
            Some("List") => parse_list(parts),
            Some("Quit") => parse_quit(parts),
            Some(_) => Err(()),
            None => Err(()),
        };

        // コマンドごとの処理を行う
        match input {
            Ok(Command::Add { name, department }) => {
                let members = company.entry(department).or_default();

                if !members.contains(&name) {
                    members.push(name);
                }

                // 本当は責務的にもパフォーマンス的にも sort するべきではないと思うが、
                // 発展的なデータ構造や immutable な list 操作を知らないため、ここで sort する。
                members.sort()
            }
            Ok(Command::List { department }) => {
                match department.as_str() {
                    "All" => {
                        // key を sort するために、vec を生成
                        let mut keys: Vec<_> = company.keys().collect();
                        keys.sort();

                        for department in keys {
                            let members = company.get(department);
                            match members {
                                Some(members) => {
                                    print_list(department, members);
                                }
                                None => {
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {
                        let members = company.get(&department);
                        match members {
                            Some(members) => {
                                print_list(&department, members);
                            }
                            None => {
                                println!("your input is missing!");
                                continue;
                            }
                        }
                    }
                };
            }
            Ok(Command::Quit) => {
                break;
            }
            Err(()) => {
                println!("your input is invalid!");
                continue;
            }
        }
    }
}

fn parse_add<'a>(mut parts: std::str::SplitWhitespace<'a>) -> Result<Command, ()> {
    let name = match parts.next() {
        Some(name) => name,
        None => return Err(()),
    };

    let Some("to") = parts.next() else {
        return Err(());
    };

    let department = match parts.next() {
        Some(department) => department,
        None => return Err(()),
    };

    // 追加のパラメータは禁止
    if parts.next().is_some() {
        return Err(());
    }

    Ok(Command::Add {
        name: name.to_string(),
        department: department.to_string(),
    })
}

fn parse_list<'a>(mut parts: std::str::SplitWhitespace<'a>) -> Result<Command, ()> {
    let department = match parts.next() {
        Some(department) => department,
        None => return Err(()),
    };

    // 追加のパラメータは禁止
    if parts.next().is_some() {
        return Err(());
    }

    Ok(Command::List {
        department: department.to_string(),
    })
}

fn parse_quit<'a>(mut parts: std::str::SplitWhitespace<'a>) -> Result<Command, ()> {
    // 追加のパラメータは禁止
    if parts.next().is_some() {
        return Err(());
    }

    Ok(Command::Quit)
}

// &[String] にするのは必須ではないが、こっちだと Vec 以外も受け取れて嬉しい。
fn print_list(department: &str, members: &[String]) {
    println!("{department}");
    for name in members {
        println!("- {name}")
    }
    println!();
}
