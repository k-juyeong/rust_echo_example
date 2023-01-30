use std::io::{self, BufRead};
use rand::Rng;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // pc random choice
    let _pc = rand::thread_rng().gen_range(1..=3);

    handle.read_line(&mut buffer)?;

    buffer = (buffer.trim_end()).to_string();

    // 1. always pc win
    // match buffer.as_str() {
    //     "rock" => println!("pc: paper"),
    //     "paper" => println!("pc: scissors"),
    //     "scissors" => println!("pc: rock"),
    //     _ => panic!("We're playing rock-paper-scissor right now"),
    // }

    // 2. pc is also player ver.1
    // 1 == Rock, 2 == Paper, 3 == Scissors
    let _user: i32 = match buffer.as_str() {
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

    // 3. using socket
    
    Ok(())
}