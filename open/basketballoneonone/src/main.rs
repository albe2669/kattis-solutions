use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let v : Vec<char> = get_input(&stdin).trim().chars().collect();
    let mut a : u32 = 0;
    let mut b : u32 = 0;
    let mut overtime : bool = false;

    for i in (0..v.len()).step_by(2) {
        let scorer : char = v[i];
        let points : u32 = v[i + 1].to_string().parse::<u32>().unwrap();

        if scorer == 'A' {
            a += points;
        } else {
            b += points;
        }

        if a == 10 && b == 10 && !overtime {
            a = 0;
            b = 0;
            overtime = true;
        } else if a >= 11 && !overtime {
            println!("A");
            break;
        } else if b >= 11 && !overtime {
            println!("B");
            break;
        } else if overtime && a >= b + 2 {
            println!("A");
            break;
        }  else if overtime && b >= a + 2 {
            println!("B");
            break;
        }
    }
}