use std::{net::Ipv6Addr, str::FromStr};

#[derive(Debug)]
struct Ipv4Addr(u8, u8, u8, u8);

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call message = {:?}", &self);
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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        },
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();

    let v4 = Ipv4Addr(127, 0, 0, 1);
    println!("v4.0 ={:?}", v4.0);
    println!("v4.1 ={:?}", v4.1);
    println!("v4.2 ={:?}", v4.2);
    println!("v4.3 ={:?}", v4.3);
    let home = IpAddr::V4(v4);
    println!("home ={:?}", home);

    let v6 = Ipv6Addr::from_str("::1").expect("REASON");
    let loopback = IpAddr::V6(v6);
    println!("loopback = {:?}", loopback);

    let penny = value_in_cents(Coin::Penny);
    println!("{penny}");

    let quarter = value_in_cents(Coin::Quarter(UsState::Alaska));

    println!("{quarter}");
}
