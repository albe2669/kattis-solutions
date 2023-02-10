use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n : Vec<f32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    let radius1 = std::f32::consts::PI * n[0].powf(2f32);
    let radius2 = std::f32::consts::PI * (n[0] - n[1]).powf(2f32);

    println!("{:.9}", radius2 / radius1 * 100f32);
}
