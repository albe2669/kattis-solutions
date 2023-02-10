use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn get_input_int(io : &std::io::Stdin) -> u32 {
    return get_input(io).trim().parse::<u32>().unwrap();
}

fn main() {
    let stdin = io::stdin();
    let n = get_input_int(&stdin);
    
    let mut t : Vec<u32> = Vec::new();
    let mut no : Vec<u32> = Vec::new();

    for _i in 0..n {
        t.push(get_input_int(&stdin));
    }

    t.sort();

    for i in 1..t[t.len() - 1] {
        if !t.iter().any(|s| *s == i) {
            no.push(i);         
        }
    }

    if no.len() > 0 {
        for i in no {
            println!("{}", i);
        }
    } else  {
        println!("good job");
    }
}
