use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: u32 = input.next().unwrap().trim().parse().unwrap();

    if n % 10 == 0 && n != 0 {
        output.write_all(b"Jebb").unwrap();
    } else {
        output.write_all(b"Neibb").unwrap();
    }
}
