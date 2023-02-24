use std::{collections::BinaryHeap, io};

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let mut sorted: BinaryHeap<char> = BinaryHeap::new();

    sorted.append(
        &mut (get_input(&stdin)
            .trim()
            .chars()
            .collect::<BinaryHeap<char>>()),
    );

    sorted.append(
        &mut (get_input(&stdin)
            .trim()
            .chars()
            .collect::<BinaryHeap<char>>()),
    );

    println!("{}", sorted.into_sorted_vec().iter().collect::<String>());
}
