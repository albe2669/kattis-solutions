use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let v : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();
    
    let mut parts : Vec<String> = Vec::new();
    let mut days : Vec<String> = Vec::new();
    
    for _i in 0..v[1] {
        let t : String = get_input(&stdin).trim().to_string();

        if !parts.contains(&t) {
            parts.push(t.clone());
        }

        days.push(t);
    }

    if parts.len() as u32 != v[0] {
        println!("paradox avoided");
    } else {
        for i in 0..days.len() {
            if parts.contains(&days[i]) {
                parts.retain(|r| *r != days[i]);
            }
    
            if parts.len() == 0 {
                println!("{}", i + 1);
                break;
            }
        }
    
        if parts.len() != 0 {
            println!("paradox avoided");
        }   
    }
}