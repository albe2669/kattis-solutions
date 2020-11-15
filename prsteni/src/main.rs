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
    let _n = get_input_int(&stdin);
    
    let t : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse::<u32>().unwrap() * 2).collect();

    for i in 1..t.len() {
        let init : u32 = t[0];
        let num : u32 = t[i];

        let gcf = common_factor(init, num);

        println!("{}/{}", init / gcf, num / gcf);        
    }
}

fn common_factor(a : u32, b : u32) -> u32 {
    for i in (1..a.min(b) + 1).rev() {
        if a % i == 0 && b % i == 0 {
            return i;
        }
    }

    return 0;
}
