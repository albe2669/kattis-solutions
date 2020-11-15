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
    let s = get_input(&stdin).trim().to_owned();

    for i in (2..n + 1).rev() {
        
        println!("{} bottles of {} on the wall, {} bottles of {}.", i, s, i, s);
        
        if i == 2 {
            println!("Take one down, pass it around, {} bottle of {} on the wall.\n", i - 1, s);
            break;
        }
        println!("Take one down, pass it around, {} bottles of {} on the wall.\n", i - 1, s);
    }

    println!("{} bottle of {} on the wall, {} bottle of {}.", 1, s, 1, s);
    println!("Take it down, pass it around, no more bottles of {}.", s);

}
