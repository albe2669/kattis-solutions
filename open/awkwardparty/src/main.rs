use std::io::{self};
use std::collections::HashMap;

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
    
    let guests : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut hash : HashMap<u32, usize> = HashMap::new();
    let mut smallest : usize = 0;
    
    for i in 0..guests.len() {
        if hash.contains_key(&guests[i]) {
            let entry = hash.get(&guests[i]).unwrap();
            if (i + 1) - entry < smallest || smallest == 0 {
                smallest = (i + 1) - entry;
            }
        }

        hash.insert(guests[i], i + 1); 
    }
   
    if smallest == 0 {
        println!("{}", n);
    } else {
        println!("{}", smallest);
    }
}
