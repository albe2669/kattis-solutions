use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> i64 {
    return get_input(io).trim().parse::<i64>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let n = get_input_int(&stdin);

    for _i in 0..n {
        let message : Vec<char> = get_input(&stdin).chars().collect();

        let mut mess : Vec<Vec<char>> = Vec::new();
        let size : u32 = (message.len() as f64).sqrt() as u32;

        mess.resize(size as usize, Vec::new());

        for j in 0..size {
            mess[j as usize].resize(size as usize, ' ');

            for k in 0..size {
                mess[j as usize][k as usize] = message[(j * size + k) as usize];
            }
        }

        for j in (0..size).rev() {
            for k in 0..size {
                print!("{}", mess[k as usize][j as usize]);
            }
        }
        println!("");
    }
}