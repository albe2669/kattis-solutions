use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.trim().split('\n');

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    input.next();

    let mut all_numbers = Vec::new();

    for line in input {
        all_numbers.extend(
            line.split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        );

        all_numbers.sort();

        output
            .write_fmt(format_args!("{}\n", all_numbers[all_numbers.len() / 2]))
            .unwrap();
    }

    output.flush().unwrap();
}
