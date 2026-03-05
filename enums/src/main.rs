#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let c = Coin::Quarter(UsState::Alabama);
    println!("{}", match_value(&c));
    println!("{}", match_value(&c));
}

fn match_value(c: &Coin) -> &str {
    match c {
        Coin::Dime => "1",
        Coin::Penny => "5",
        Coin::Nickel => "10",
        Coin::Quarter(state) => match state {
            UsState::Alaska => "Alaska baby",
            UsState::Alabama => "Alabamamama",
        },
    }
}
