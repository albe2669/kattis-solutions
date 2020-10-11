use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let t : Vec<char> = get_input(&stdin).trim().chars().collect();

    let mut white : f64 = 0.0;
    let mut lower : f64 = 0.0;
    let mut upper : f64 = 0.0;
    let mut symbo : f64 = 0.0;

    for i in 0..t.len() {
        let c : char = t[i];
        if c == '_' {
            white += 1.0;
        } else if c.is_ascii_lowercase() {
            lower += 1.0;
        } else if c.is_ascii_uppercase() {
            upper += 1.0;
        } else {
            symbo += 1.0;
        }
    }

    println!("{}", white / t.len() as f64);
    println!("{}", lower / t.len() as f64);
    println!("{}", upper / t.len() as f64);
    println!("{}", symbo / t.len() as f64);
}