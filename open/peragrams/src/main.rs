use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n = get_input(&stdin).trim().to_owned();
   
    let mut alph : Vec<u32> = vec![0; 26];
    
    for c in n.chars() {
        alph[gib(c) as usize] += 1;
    }

    let mut c = 0;

    for a in alph {
        if a != 0 && a % 2 == 1 {
            c += 1;
        }
    }

    if c != 0 {
        println!("{}", c - 1);
    } else {
        println!("{}", c);
    }
}

fn gib(c : char) -> u32 {
    return (c as u32) - ('a' as u32);
}
