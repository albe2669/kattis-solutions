use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: usize = input.next().unwrap().trim().parse().unwrap();
    let m: usize = input.next().unwrap().trim().parse().unwrap();

    let mut snow = vec![0; m];

    for _i in 0..n {
        let snow_chars = input.next().unwrap().chars().collect::<Vec<char>>();

        for j in 0..m {
            if snow_chars[j] == 'S' {
                snow[j] += 1;
            }
        }
    }

    for i in (0..n).rev() {
        (0..m).for_each(|j| {
            if snow[j] <= i {
                output.write_all(b".").unwrap();
            } else {
                output.write_all(b"S").unwrap();
            }
        });
        output.write_all(b"\n").unwrap();
    }

    output.flush().unwrap();
}
