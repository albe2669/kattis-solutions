use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let a: u32 = input.next().unwrap().trim().parse().unwrap();
    let b: u32 = input.next().unwrap().trim().parse().unwrap();

    if a > b {
        output.write_all(b"MAGA!").unwrap();
    } else if a < b {
        output.write_all(b"FAKE NEWS!").unwrap();
    } else {
        output.write_all(b"WORLD WAR 3!").unwrap();
    }
}
