use std::net::UdpSocket;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:3401").expect("bind error");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        
        match line.as_str() {
            "rock" | "Rock" | "paper" | "Paper" | "scissors" | "Scissors" =>
                print!("Who's the winner: "),
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