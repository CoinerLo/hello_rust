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
}
