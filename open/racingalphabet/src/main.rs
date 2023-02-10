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
        let t : Vec<char> = get_input(&stdin).trim().chars().collect();
        let mut dist : f64 = 0.0;

        for j in 0..t.len() - 1 {
            let s : i64 = match t[j] {
                ' ' => 26,
                '\'' => 27,
                _ => t[j] as i64 - ('A' as i64)
            };

            let d : i64 = match t[j+1] {
                ' ' => 26,
                '\'' => 27,
                _ => t[j+1] as i64 - ('A' as i64)
            };

            let diff : i64 = if s - d < 0 { d - s } else { s - d };
            
            dist += if diff > 14 { (28 - diff) as f64 } else { diff as f64 };
        }
        println!("{:.10}", dist * (60f64 * std::f64::consts::PI) / 420f64 + (t.len() as f64));
    }
}

