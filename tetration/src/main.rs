use std::io::{self};

fn get_input_int(io : &std::io::Stdin) -> f64 {
    return get_input(io).trim().parse().unwrap();
}

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let n : f64 = get_input_int(&stdin);
    
    println!("{:.6}", n.powf(1f64 / n));
}