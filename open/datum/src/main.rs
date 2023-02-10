use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let n : Vec<i64> = get_input(&stdin).split_whitespace().map(|n| n.parse().unwrap()).collect();

    run(n[0], n[1], 2009);
}

fn run(d : i64, m : i64, y : i64) {
    match calc(d, m, y) {
        0 => println!("Sunday"),
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        _ => println!("That failed")
    }
}

fn calc(d : i64, m : i64, y : i64) -> i64 {
    let new_m : i64;
    let tmp_y : i64;
    let new_c : i64;
    let new_y : i64;
    
    if m == 1 {
        new_m = 11;
        tmp_y = y - 1;
    } else if m == 2 {
        new_m = 12;
        tmp_y = y - 1;
    } else {
        new_m = m - 2;
        tmp_y = y;
    }
    
    new_y = tmp_y % 100;
    new_c = tmp_y / 100;

    let month : i64 = (2.6 * (new_m as f64) - 0.2).floor() as i64;
    let year : i64 = new_y / 4;
    let century : i64 = (new_c / 4) - 2 * new_c;
    let res : i64 = (d + month + new_y + year + century) % 7;

    if res < 0 {
        return res + 7;
    }

    return res;
}
