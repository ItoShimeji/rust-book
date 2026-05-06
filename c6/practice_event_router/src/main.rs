enum AppEvent {
    Login { user: String },
    Message { from: String, text: String },
    Warning(String),
    Logout,
    Noop,
}

struct RouterState {
    logged_in_user: Option<String>,
}

impl RouterState {
    fn new() -> Self {
        Self {
            logged_in_user: None,
        }
    }
}

fn main() {
    let mut state = RouterState::new();

    let events = [
        AppEvent::Login {
            user: "Aki".to_string(),
        },
        AppEvent::Message {
            from: "Aki".to_string(),
            text: "hello".to_string(),
        },
        AppEvent::Warning("disk almost full".to_string()),
        AppEvent::Logout,
        AppEvent::Noop,
    ];

    for event in events {
        let message = handle_event(&mut state, event);
        println!("{message}")
    }
}

fn handle_event(state: &mut RouterState, event: AppEvent) -> String {
    match event {
        AppEvent::Login { user } => {
            // ここは所有権エラーを起こさないように message を返すための処理と login user の登録処理が交互になっている
            // move する前とした後はこのように順序を分離して書くパターンが Rust では必要なのか
            // format! が借用を暗黙的に受け取るのはマクロの影響か？
            let message = format!("{user} is logged in");

            state.logged_in_user = Some(user);
            message
        }
        AppEvent::Message { from, text } => {
            if text.is_empty() {
                format!("empty message from {}", from)
            } else {
                format!("{}: {}", from, text)
            }
        }
        AppEvent::Logout => {
            // take() によって、Some() を None にして取り出す
            // Some の値の所有権が左辺に移る
            if let Some(user) = state.logged_in_user.take() {
                let message = format!("{user} is logged out");
                message
            } else {
                String::from("logout is failure because there is no logged in user")
            }
        }
        AppEvent::Warning(warning) => format!("warning: {warning}"),
        AppEvent::Noop => String::from("Noop!"),
    }
}
