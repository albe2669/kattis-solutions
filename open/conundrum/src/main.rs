use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let cipher : Vec<char> = get_input(&stdin).trim().to_ascii_lowercase().chars().collect();

    let mut days : u32 = 0;
    let mut n : usize = 0;

    for i in 0..cipher.len() {
        n += 1;
        
        if n == 4 {
            n = 1;
        }

        if n == 1 && cipher[i] != 'p' {
            days += 1;
        }

        if n == 2 && cipher[i] != 'e' {
            days += 1;
        }

        if n == 3 && cipher[i] != 'r' {
            days += 1;
        }

    }

    println!("{}", days);
}