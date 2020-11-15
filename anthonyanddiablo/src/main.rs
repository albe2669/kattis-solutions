use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n : Vec<f64> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    if 2f64 * std::f64::consts::PI * (n[0]/std::f64::consts::PI).sqrt() <= n[1] {
        println!("Diablo is happy!");
    } else {
        println!("Need more materials!");
    }
}
