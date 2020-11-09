use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let first : i64 = get_input(&stdin).trim().chars().filter(|c| *c == 'a').collect::<Vec<char>>().len() as i64;
    let second : i64 = get_input(&stdin).trim().chars().filter(|c| *c == 'a').collect::<Vec<char>>().len() as i64;

    if second > first {
        println!("no");
    } else {
        println!("go");
    }
}