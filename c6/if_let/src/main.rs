#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            // ...
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        }
    }

    {
        let coin = Coin::Quarter(UsState::Alaska);
        if let Some(description) = descrive_state_qurter(coin) {
            println!("{description}");
        }
    }

    {
        let coin = Coin::Penny;
        if let None = descrive_state_qurter(coin) {
            println!("this coin isn't Quarter");
        };
    }
}

fn descrive_state_qurter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}
