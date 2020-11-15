use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    if n[1] - n[0] > n[2] - n[1] {
        println!("{}", n[1] - n[0] - 1);
    } else {
        println!("{}", n[2] - n[1] - 1);
    }
}
