use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> u32 {
    return get_input(io).trim().parse::<u32>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let n : u32 = get_input_int(&stdin);

    for _i in 0..n  {
        let s : Vec<f64> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();
        
        println!("{}", area(s[0], s[1], s[2], s[3]));

    }
}

fn area(n : f64, l : f64, d : f64, g : f64) -> f64 {
    return  (0.25 * n * l.powf(2f64) * (1f64 / (std::f64::consts::PI / n).tan())) +
            (n * l * d * g) +
            ((g * d).powf(2f64) * std::f64::consts::PI);
}