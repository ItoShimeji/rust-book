struct Login {
    user: String,
}

struct Message {
    from: String,
    text: String,
}

enum AppEvent {
    Login(Login),
    Message(Message),
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
        AppEvent::Login(Login {
            user: "Aki".to_string(),
        }),
        AppEvent::Message(Message {
            from: "Aki".to_string(),
            text: "hello".to_string(),
        }),
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
        AppEvent::Login(event) => {
            let user = &event.user[..];
            let message = format!("{user} is logged in");
            state.logged_in_user = Some(event.user);
            message
        }
        AppEvent::Message(message) => {
            if message.text.is_empty() {
                format!("empty message from {}", message.from)
            } else {
                format!("{}: {}", message.from, message.text)
            }
        }
        AppEvent::Logout => {
            if let Some(user) = &mut state.logged_in_user {
                let message = format!("{user} is logged out");
                state.logged_in_user = None;
                message
            } else {
                String::from("logout is failure because there is no logged in user")
            }
        }
        AppEvent::Warning(warning) => format!("warning: {warning}"),
        AppEvent::Noop => String::from("Noop!"),
    }
}
