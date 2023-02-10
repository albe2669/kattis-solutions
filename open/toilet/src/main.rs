use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let positions : Vec<char> = get_input(&stdin).trim().chars().collect();
    
    println!("{}\n{}\n{}", up(positions.clone()), down(positions.clone()), leave(positions.clone()));
}

fn up(positions : Vec<char>) -> u32{
    let mut score = 0;
    let mut prev = positions[0];

    for i in 1..positions.len() {
        let c = positions[i];
        
        if prev == 'D' {
            score += 1;
        } else if prev == 'U' && c == 'D' {
            score += 2;
        }

        prev = 'U';
    }

    return score;
}

fn down(positions : Vec<char>) -> u32{
    let mut score = 0;
    let mut prev = positions[0];

    for i in 1..positions.len() {
        let c = positions[i];
        
        if prev == 'U' {
            score += 1;
        } else if prev == 'D' && c == 'U' {
            score += 2;
        }

        prev = 'D';
    }

    return score;
}

fn leave(positions : Vec<char>) -> u32{
    let mut score = 0;
    let mut prev = positions[0];

    for i in 1..positions.len() {
        let c = positions[i];
        
        if prev != c {
            score += 1;
        }

        prev = c;
    }

    return score;
}

