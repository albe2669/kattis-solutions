use std::io::{self, Read, Write};

fn main() {
    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    output.write_all(b"Hipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\nHipp hipp hurra!\n").unwrap();
}
