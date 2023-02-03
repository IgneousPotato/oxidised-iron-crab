enum IpAddrKind {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) -> &Message{
        dbg!(self)
    }
}

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
    enumss();
    matches();
    iflet();
}

fn enumss() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn matches() {
    let ccoin = Coin::Penny;
    value_in_cents(ccoin);

    let scoin = Coin::Quarter(UsState::Alabama);
    value_in_cents(scoin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("PENNNNNYYYYYYYYYYYY!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn iflet() {
    let config_max = Some(3u8);
    
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
