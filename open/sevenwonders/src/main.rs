use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let string : Vec<char> = get_input(&stdin).trim().chars().collect();

    let mut t : u32 = 0;
    let mut c : u32 = 0;
    let mut g : u32 = 0;

    for card in string {
        match card {
            'T' => t += 1,
            'C' => c += 1,
            'G' => g += 1,
            _ => t += 0
        }
    }
    
    println!("{}", t.pow(2) + c.pow(2) + g.pow(2) + 7 * t.min(c.min(g)));
}