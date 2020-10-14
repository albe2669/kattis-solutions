use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

fn main() {
    let stdin = io::stdin();

    let tex : Vec<char> = get_input(&stdin).trim().to_ascii_lowercase().chars().collect();
    let key : Vec<char> = get_input(&stdin).to_ascii_lowercase().trim().chars().collect();

    for i in 0..tex.len() {
        let diff : i64;
        if i % 2 != 0 {
            diff = (to_int(tex[i]) + to_int(key[i])) as i64;
        } else {
            diff = to_int(tex[i]) as i64 - to_int(key[i]) as i64;
        }

        if diff >= 26 {
            print!("{}", ASCII_LOWER[(diff - 26) as usize].to_ascii_uppercase());
        } else if diff < 0 {
            print!("{}", ASCII_LOWER[(diff + 26) as usize].to_ascii_uppercase());
        } else {
            print!("{}", ASCII_LOWER[diff as usize].to_ascii_uppercase());
        }
    }

    println!("");
}

fn to_int(n : char) -> u32 {
    return ASCII_LOWER.iter().position(|&r| r == n).unwrap() as u32;
}