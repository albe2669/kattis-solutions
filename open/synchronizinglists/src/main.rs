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
    
    loop {
        let n : u32 = get_input_int(&stdin) as u32;

        if n == 0 {
            break;
        }

        let mut l1 : Vec<i64> = Vec::new();
        let mut l2 : Vec<i64> = Vec::new();
        let mut l3 : Vec<i64> = Vec::new();
        let mut l4 : Vec<i64> = vec![Default::default(); n as usize];
        
        for _i in 0..n {
            l1.push(get_input_int(&stdin));
        }

        for _i in 0..n {
            l2.push(get_input_int(&stdin));
        }

        for i in 0..l1.len() {
            l3.push(l1[i]);
        }
			
        l2.sort();
        l3.sort();

        for i in 0..l3.len() {
            let t = l1.iter().position(|&s| s == l3[i]).unwrap();

            l4[t] = l2[i];
        }

        for i in l4 {
            println!("{}", i);
        }
        println!("");
    }
    
    
}