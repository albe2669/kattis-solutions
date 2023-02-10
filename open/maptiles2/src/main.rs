use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let s : Vec<char> = get_input(&stdin).trim().chars().collect();

    let mut x : i64 = 0; 
    let mut y : i64 = 0; 
    
    let l : i64 = s.len() as i64;

    for i in 0..s.len() {
        if s[i] == '1' || s[i] == '3' {
            x += 1 << (l - (i as i64) - 1);
        }
        if s[i] == '2' || s[i] == '3' {
            y += 1 << (l - (i as i64) - 1);
        }
    }

    println!("{} {} {}", l, x, y);
}

