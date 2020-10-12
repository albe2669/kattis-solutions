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
    get_input_int(&stdin);

    let scales : Vec<Vec<u32>> = vec![
        vec![0,2,4,5,7,9,11],
        vec![1,3,5,6,8,10,0],
        vec![2,4,6,7,9,11,1],
        vec![3,5,7,8,10,0,2],
        vec![4,6,8,9,11,1,3],
        vec![5,7,9,10,0,2,4],
        vec![6,8,10,11,1,3,5],
        vec![7,9,11,0,2,4,6],
        vec![8,10,0,1,3,5,7],
        vec![9,11,1,2,4,6,8],
        vec![10,0,2,3,5,7,9],
        vec![11,1,3,4,6,8,10]
    ];

    let mut notes : Vec<bool> = vec![false; 12];

    for note in get_input(&stdin).trim().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>().iter() {
        notes[num(note)] = true;
    }

    let mut printed : bool = false;
    for i in 0..scales.len() {
        if scale(&notes, &scales[i]) {
            printed = true;
            print!("{} ", note(&(i as u32)));
        }
    }

    if !printed {
        print!("none");
    }

    println!("");
}

fn num(note : &str ) -> usize {
    return match note {
        "A" => 0,
        "A#" => 1,
        "B" => 2,
        "C" => 3,
        "C#" => 4,
        "D" => 5,
        "D#" => 6,
        "E" => 7,
        "F" => 8,
        "F#" => 9,
        "G" => 10,
        "G#" => 11,
        _ => 0
    }
}

fn note(n : &u32) -> &str {
    return match n {
        0 => "A",
        1 => "A#",
        2 => "B",
        3 => "C",
        4 => "C#",
        5 => "D",
        6 => "D#",
        7 => "E",
        8 => "F",
        9 => "F#",
        10 => "G",
        11 => "G#",
        _ => ""
    }
}

fn scale(notes : &Vec<bool>, scale : &Vec<u32>) -> bool {
    let mut works : bool = true;
    for i in 0..notes.len() {
        let mut inthisscale : bool = false;

        for j in 0..scale.len() {
            if scale[j] == i as u32 {
                inthisscale = true;
            }
        }

        if notes[i] && !inthisscale {
            works = false;
        }
    }

    return works;
}