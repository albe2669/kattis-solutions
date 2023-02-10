use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    loop {
        let s : Vec<f64> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

        if s[0] == 0f64 && s[1] == 0f64 && s[2] == 0f64 {
            break;
        }

        println!("{} {}", std::f64::consts::PI * s[0].powf(2f64), (s[2] / s[1])* (s[0] * 2f64).powf(2f64));
    }
}