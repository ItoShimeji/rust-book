struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user1's name is {}", user1.username);

    let user2 = build_user(
        String::from("rust_beginner"),
        String::from("momonga@rust.org"),
    );

    println!("user2's email is {}", user2.email);

    // 明示的に更新したい field のみ最初に定義
    // 残りは展開
    // js のような定義順による暗黙的な上書きは禁止
    let user3 = User {
        email: String::from("archeopteryx@rust.org"),
        ..user2
    };

    println!("user3's email is {}", user3.email);

    // user2 のフィールドは user3 の定義で move されており、エラーとなる
    // let user4 = User{
    //     email: String::from("t-lex@rust.org"),
    //     ..user2
    // };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
