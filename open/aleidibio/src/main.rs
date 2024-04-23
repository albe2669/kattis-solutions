use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let numbers: Vec<u32> = input.map(|x| x.parse().unwrap()).collect::<Vec<u32>>();

    output
        .write_fmt(format_args!("{}\n", numbers[2] - numbers[1] - numbers[0]))
        .unwrap();
}
