use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    println!("{}", if get_input(&stdin).trim().chars().filter(|c| *c == 'a').collect::<Vec<char>>().len() < get_input(&stdin).trim().chars().filter(|c| *c == 'a').collect::<Vec<char>>().len() {"no" } else { "go" });
}