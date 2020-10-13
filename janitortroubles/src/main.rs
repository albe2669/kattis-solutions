use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n : Vec<f64> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

    let perimiter : f64 = (n[0] + n[1] + n[2] + n[3]) / 2f64;

    println!("{}", 
        ((perimiter - n[0]) *
        (perimiter - n[1]) *
        (perimiter - n[2]) *
        (perimiter - n[3])).sqrt());
}