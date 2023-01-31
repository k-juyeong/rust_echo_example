use std::io::{self, BufRead};
use rand::Rng;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    
    // pc random choice
    let _pc = rand::thread_rng().gen_range(1..=3);

    // 3. using socket
    let socket = UdpSocket::bind("127.0.0.1:3400").expect("bind error");
    let mut buf = [0; 1024];

    let (number_of_bytes, addr) = socket.recv_from(&mut buf).expect("recv_from error");

    let message = std::str::from_utf8(&buf[..number_of_bytes]).unwrap();
    println!("{:?}", message);

    // 1 == Rock, 2 == Paper, 3 == Scissors
    let _user: i32 = match message {
        "rock" | "Rock" => 1,
        "paper" | "Paper" => 2,
        "scissors" | "Scissors" => 3,
        &_ => panic!("We're playing rock-paper-scissor right now"), 
    };

    if _pc == _user {println!("draw");}
    
    match (_pc, _user) {
        (1, 2) => println!("user win!"),
        (1, 3) => println!("pc win!"),
        (2, 1) => println!("pc win!"),
        (2, 3) => println!("user win!"),
        (3, 1) => println!("user win!"),
        (3, 2) => println!("pc win!"),
        _ => unreachable!("Wrong"),
    }

    Ok(())
}