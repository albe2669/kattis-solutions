use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> f32 {
    return get_input(io).trim().parse::<f32>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let n : Vec<f32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    let diag = (n[1].powf(2f32) + n[2].powf(2f32)).sqrt();

    for _i in 0..(n[0] as u32) {
        let t = get_input_int(&stdin);
        
        if t <= n[1] || t <= n[2] || t <= diag {
            println!("DA");
        } else {
            println!("NE");
        }
    }
}
