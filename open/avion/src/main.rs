use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();
    let mut total : bool = false;

    for i in 0..5 {
        let t = get_input(&stdin);

        if t.contains("FBI") {
            total = true;
                
            print!("{}", i + 1);

            if i != 4 {
                print!(" ");
            }
        }
    }

    if !total {
        println!("HE GOT AWAY!");
    } else {
        println!("");
    }
}