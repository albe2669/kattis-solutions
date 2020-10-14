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
    let p : u32 = get_input_int(&stdin);
    let ans : Vec<char> = get_input(&stdin).trim().chars().collect();

    let mut a : u32 = 0;
    let mut b : u32 = 0;
    let mut g : u32 = 0;

    let a_s : Vec<char> = vec!['A','B','C'];
    let b_s : Vec<char> = vec!['B','A','B','C'];
    let g_s : Vec<char> = vec!['C','C','A','A','B','B'];

    for i in 0..p {
        let an = ans[i as usize];

        a += if a_s[(i % 3) as usize] == an { 1 } else { 0 };
        b += if b_s[(i % 4) as usize] == an { 1 } else { 0 };
        g += if g_s[(i % 6) as usize] == an { 1 } else { 0 };
    }

    let highest : u32 = a.max(b.max(g));

    println!("{}", highest);

    if a == highest {
        println!("Adrian");
    }
    if b == highest {
        println!("Bruno");
    }
    if g == highest {
        println!("Goran");
    }
}