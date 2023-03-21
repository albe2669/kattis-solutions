use std::io::{self, Read, Write};

// Fenwick tree
struct FenwickTree {
    tree: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> FenwickTree {
        FenwickTree {
            tree: vec![0; n + 5],
        }
    }

    fn sum(&self, mut i: usize) -> i64 {
        let mut s = 0;

        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }

    fn update(&mut self, n: usize, mut i: usize, x: i64) {
        i += 1;

        while i < (n + 5) {
            self.tree[i] += x;
            i += i & i.wrapping_neg();
        }
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: usize = input.next().unwrap().parse().unwrap();
    let q: u32 = input.next().unwrap().parse().unwrap();

    let mut ft = FenwickTree::new(n);

    for _ in 0..q {
        match input.next().unwrap() {
            "+" => {
                let i = input.next().unwrap().parse::<usize>().unwrap();
                let d = input.next().unwrap().parse::<i64>().unwrap();
                ft.update(n, i, d);
            }
            "?" => {
                let i = input.next().unwrap().parse::<usize>().unwrap();
                output.write_fmt(format_args!("{}\n", ft.sum(i))).unwrap();
            }
            _ => {}
        }
    }
}
