use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();

    if n[0] == 0 || n[1] == 0 || n[2] == 0 || n[3] < 3 {
        println!("NO");
        
        return;
    }

    if n[0] + n[1] + n[2] >= n[3] {
        println!("YES");
    } else {
        println!("NO");
    }
}
