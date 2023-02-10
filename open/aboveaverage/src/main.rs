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
    
    for _i in 0..n {
        let students : Vec<f64> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
            
        let mut avg = 0f64;
        for i in 1..students.len() {
            avg += students[i];
        }

        avg /= students[0];
        let mut count = 0f64;

        for i in 1..students.len() {
            if students[i] > avg {
                count += 1f64;
            }
        }

        println!("{:.3}%", count / students[0] * 100f64);
        
    }
}
