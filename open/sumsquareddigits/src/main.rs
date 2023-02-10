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
    let p : u32 = get_input_int(&stdin) as u32;

    for _i in 0..p {
        // data[0] = k
        // data[1] = b
        // data[2] = n
        let data : Vec<u32> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

        let mut ans : u32 = 0;
        let mut n : u32 = data[2];
        
        while n > 0 {
            ans += (n % data[1]) * (n % data[1]);
            n /= data[1];
        }

        println!("{} {}", data[0], ans);
    }
    
    
}