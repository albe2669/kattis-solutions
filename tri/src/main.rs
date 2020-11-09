use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let i : Vec<i64> = get_input(&stdin).trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    if add(i[0], i[1], i[2]) {

    } else if ret(i[0], i[1], i[2]) {

    } else if mul(i[0], i[1], i[2]) {

    } else if div(i[0] as f64, i[1] as f64, i[2] as f64) {

    }
}

fn div(a : f64, b : f64, c : f64) -> bool {
    if a / b == c {
        println!("{}/{}={}", a, b, c);
        return true;

    } else if a == b/c {
        println!("{}={}/{}", a, b, c);
        return true;
    }

    return false;
}

fn add(a : i64, b : i64, c : i64) -> bool {
    if a + b == c {
        println!("{}+{}={}", a, b, c);
        return true;

    } else if a == b + c {
        println!("{}={}+{}", a, b, c);
        return true;
        
    }

    return false;
}

fn ret(a : i64, b : i64, c : i64) -> bool {
    if a - b == c {
        println!("{}+{}={}", a, b, c);
        return true;

    } else if a == b - c {
        println!("{}={}-{}", a, b, c);
        return true;
        
    }

    return false;
}

fn mul(a : i64, b : i64, c : i64) -> bool {
    if a * b == c {
        println!("{}*{}={}", a, b, c);
        return true;

    } else if a == b * c {
        println!("{}={}*{}", a, b, c);
        return true;
        
    }

    return false;
}