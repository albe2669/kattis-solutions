use std::io::{self, BufRead, Write};

fn main() {
    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    for line in io::stdin().lock().lines() {
        match line.unwrap().to_lowercase().contains("problem") {
            true => output.write_all(b"yes\n").unwrap(),
            false => output.write_all(b"no\n").unwrap(),
        }
    }
}
