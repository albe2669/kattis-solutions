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
    let n : u32 = get_input_int(&stdin);

    for _i in 0..n {
        let t : String = String::from(get_input(&stdin).trim());
        let j : String = String::from(get_input(&stdin).trim());
        
        println!("{}", t);
        println!("{}", j);

        let t : Vec<char> = t.chars().collect();
        let j : Vec<char> = j.chars().collect();

        for i in 0..t.len() {
            if t[i] != j[i] {
                print!("*");
            } else {
                print!(".");
            }
        }

        println!("\n");
    }
}