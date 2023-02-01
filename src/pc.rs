use std::io::{self, BufRead};
use rand::Rng;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // 3. using socket

    // pc random choice
    let _pc = rand::thread_rng().gen_range(1..=3);

    let socket = UdpSocket::bind("127.0.0.1:3400").expect("bind error");
    let mut buf = [0; 1024];
    let mut msgbuf = &b"msg"[..];

    let (number_of_bytes, addr) = socket.recv_from(&mut buf).expect("recv_from error");

    let message = std::str::from_utf8(&buf[..number_of_bytes]).unwrap();
    println!("{:?}", message);

    // Rock == 1, Paper == 2, Scissors == 3
    let _user: i32 = match message {
        "rock" | "Rock" => 1,
        "paper" | "Paper" => 2,
        "scissors" | "Scissors" => 3,
        &_ => panic!("We're playing rock-paper-scissor right now"),
    };

    // Result
    if _pc == _user {msgbuf = &b"draw"[..];}

    match (_pc, _user) {
        (1, 2) => msgbuf = &b"user win"[..],
        (1, 3) => msgbuf = &b"pc win"[..],
        (2, 1) => msgbuf = &b"pc win"[..],
        (2, 3) => msgbuf = &b"user win"[..],
        (3, 1) => msgbuf = &b"user win"[..],
        (3, 2) => msgbuf = &b"pc win"[..],
        _ => println!("Wrong"),
    }

    // Send result to user
    socket.send_to(msgbuf, &addr).expect("send_to error");

    Ok(())
}