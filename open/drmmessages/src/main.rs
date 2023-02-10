use std::io::{self};

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

fn main() {
    let stdin = io::stdin();

    let mut v : Vec<char> = get_input(&stdin).trim().to_ascii_lowercase().chars().collect();
    let len = v.len();
    let (t, t2) : (&mut [char], &mut [char]) = v.split_at_mut(len / 2);
    let mut beginning : Vec<char> = t.to_vec();
    let mut end : Vec<char> = t2.to_vec();

    let mut rot1 : u32 = calc_rot_val(&beginning);
    let mut rot2 : u32 = calc_rot_val(&end);

    for i in 0..beginning.len() {
        beginning[i] = rotate(&beginning[i], rot1);
        end[i] = rotate(&end[i], rot2);

        print!("{}", rotate_by_char(&beginning[i], &end[i]).to_ascii_uppercase());
    }
    
    println!("");
}

fn calc_rot_val(string : &Vec<char>) -> u32 {
    let mut rot : u32 = 0;

    for i in string {
        rot += to_int(*i);
    }
    
    return rot % 26;
}

fn rotate_by_char(n : &char, b : &char) -> char {
    return ASCII_LOWER[((to_int(*n) + to_int(*b)) % 26) as usize];
}

fn rotate(n : &char, rot : u32) -> char {
    return ASCII_LOWER[((to_int(*n) + rot) % 26) as usize];
}

fn to_int(n : char) -> u32 {
    return ASCII_LOWER.iter().position(|&r| r == n).unwrap() as u32;
}