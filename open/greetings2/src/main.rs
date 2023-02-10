use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let e = get_input(&stdin).trim().chars().filter(|s| *s == 'e').collect::<Vec<char>>().len();
    
    println!("{}{}{}", 'h', std::iter::repeat('e').take(e * 2).collect::<String>(), 'y');
}
