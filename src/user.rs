use std::net::UdpSocket;
use std::io::{self, BufRead};
use std::env;

fn main() -> std::io::Result<()> {
    // let host_arg = env::args().nth(1).unwrap();

    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind error");

    println!("Let's play rock-paper-scissor!! Your opponent is PC.");
    println!("input your choice(q for quit)");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        if &line == "Q" || &line == "q" {
            break;
        }

        match line.as_str() {
            "rock" | "Rock" | "paper" | "Paper" | "scissors" | "Scissors" =>
                println!("Wait for result.."),
            _ => panic!("We're playing rock-paper-scissor now"),
        }

        socket.send_to(line.as_bytes(), "127.0.0.1:3400").expect("send_to error");

        let mut buf = [0; 1024];
        let (number_of_bytes, addr) = socket.recv_from(&mut buf).expect("recv_from error");

        let result = std::str::from_utf8(&buf[..number_of_bytes]).unwrap();
        println!("{}", result);
    }

    Ok(())
}