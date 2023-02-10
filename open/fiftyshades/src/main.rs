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
    let mut c : u32 = 0;

    for _i in 0..n {
        let t : String = get_input(&stdin).to_ascii_lowercase();
        
        if t.contains("pink") || t.contains("rose") {
            c += 1;
        }
    }
    
    if c != 0 {
        println!("{}", c);
    } else {
        println!("I must watch Star Wars with my daughter");
    }
}