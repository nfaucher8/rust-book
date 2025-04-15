enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware
}

impl Message {
    fn call(&self) {

    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}, {:?}, {:?}", five, six, none);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max is {}", max),
        None => (),
    }

    if let Some(max) = config_max {
        println!("The max is {}", max);
    }

    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;
    match &coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    if let Coin::Quarter(state) = &coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    let coin2 = Coin::Dime;

    if let Coin::Quarter(state) = &coin2 {
        println!("State quarter from {:?}!", state);
    } else if let Coin::Dime = &coin2 {
        println!("Dime");
    } else {
        count += 1;
    }
}

fn route (ip_kind: IpAddr) {

}

fn value_in_cents (coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}