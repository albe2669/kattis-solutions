
use std::io;

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let n : u32 = get_input(&stdin)
        .trim()
        .parse()
        .unwrap();

    let mut sum : u32 = 0;
    let mut start_time : u32 = 0;

    let mut is_started : bool = false;

    for _ in 0..n {
        let x : u32 = get_input(&stdin)
            .trim()
            .parse()
            .unwrap();

        is_started = !is_started;
        // Just started
        if is_started {
            start_time = x;
        } else {
            sum += x - start_time;
        }
    }

    if is_started {
        println!("still running");
    } else {
        println!("{}", sum);
    }
}
