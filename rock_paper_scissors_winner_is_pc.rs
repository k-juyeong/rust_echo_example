use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;

    println!("message: {:?}", buffer);
    match buffer.as_str() {
        "rock\n" | "paper\n" | "scissors\n" =>
            println!("pc win!!"),
        _ => panic!("We're playing rock-paper-scissor right now"),
    }

    Ok(())
}