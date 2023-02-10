use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n = get_input(&stdin).trim().to_owned();
    
    println!("AWH{}", std::iter::repeat("O").take(n.len()).collect::<String>());
}
