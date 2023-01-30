use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // pc random choice

    handle.read_line(&mut buffer)?;

    buffer = (buffer.trim_end()).to_string();

    match buffer.as_str() {
        "rock" => println!("pc: paper"),
        "paper" => println!("pc: scissors"),
        "scissors" => println!("pc: rock"),
        _ => panic!("We're playing rock-paper-scissor right now"),
    }

    Ok(())
}