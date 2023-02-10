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
    let mut enter : Vec<String> = Vec::new();    
    
    for _i in 0..n {
        let employee : Vec<String> = get_input(&stdin).trim().split_whitespace().map(|s| s.to_owned()).collect();
        
        if employee[0] == "entry" {
            print!("{} entered", employee[1]);

            if enter.iter().any(|i| *i == employee[1]) {
                println!(" (ANOMALY)");
            } else {
                println!("");
                enter.push(employee[1].clone());
            }
        } else {
            print!("{} exited", employee[1]);

            if !enter.iter().any(|i| *i == employee[1]) {
                println!(" (ANOMALY)");
            } else {
                enter.remove(enter.iter().position(|x| *x == employee[1]).unwrap());
                println!("");
            }
        }
    }
}
