use std::io::{self};
use std::collections::BTreeMap;

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
    let n : u32 = get_input_int(&stdin) as u32;
    let mut map: BTreeMap<u32, String> = BTreeMap::new();

    for _i in 0..n {
        let mut t : Vec<String> = get_input(&stdin).trim().split_whitespace().map(|s| s.to_string()).collect();
        match t[0].parse::<u32>() {
            Ok(number)  => map.insert(number / 2, t.remove(1)),
            Err(_e) => map.insert(t[1].parse::<u32>().unwrap(), t.remove(0)),
        };
    }
    
    for (_key, value) in map.into_iter() {
        println!("{}", value);            
    }
}