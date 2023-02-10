use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> f64 {
    return get_input(io).trim().parse::<f64>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    
    println!("{}", ((get_input_int(&stdin) as f64) * (1000.0 * (5280.0 / 4854.0))).round() as u32);
}