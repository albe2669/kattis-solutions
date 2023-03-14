use std::{
    cmp::{max, min},
    io::{self, Read},
};

fn main() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_to_string(&mut buffer).unwrap();

    let mut numbers = buffer.split_whitespace().map(|s| s.parse::<i32>().unwrap());

    let t = numbers.next().unwrap();

    for _ in 0..t {
        let l = numbers.next().unwrap();
        let n = numbers.next().unwrap();

        let mut low = 0;
        let mut high = 0;

        for i in 0..n {
            let x = numbers.next().unwrap();
            if i == 0 {
                low = min(x, l - x);
                high = max(x, l - x);
                continue;
            }

            low = max(low, min(x, l - x));
            high = max(high, max(x, l - x));
        }

        println!("{} {}", low, high);
    }
}
