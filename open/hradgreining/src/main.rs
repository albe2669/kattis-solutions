use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    if buffer.contains("COV") {
        output.write_all(b"Veikur!").unwrap();
    } else {
        output.write_all(b"Ekki veikur!").unwrap();
    }
}
