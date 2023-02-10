use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n = get_input(&stdin).trim().to_owned();
        
    if n.len() < 7 {
        let mut int : i64 = n.parse().unwrap();
        
        if int <= 1 {
            println!("{}", 1);
            return;
        }

        let mut div : i64 = 0;
        
        while int != 1 {
            div += 1;
            int /= div;
        }

        println!("{}", div);
    } else {
        let mut i : u64 = 1;
        let mut x : f64 = 1f64;
        loop {
            x += (i as f64).log10();
            if x.floor() == n.len() as f64 {
                println!("{}", i);
                break;
            }

            i += 1;
        }
    }
}
