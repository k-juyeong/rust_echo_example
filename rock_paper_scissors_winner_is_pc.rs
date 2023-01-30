use std::io::{self, BufRead};
// use substring::Substring;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;

    buffer = (&buffer[..buffer.len()-1]).to_string();

    match buffer.as_str() {
        "rock" => println!("pc: paper"),
        "paper" => println!("pc: scissors"),
        "scissors" => println!("pc: rock"),
        _ => panic!("We're playing rock-paper-scissor right now"),
    }

    Ok(())
}