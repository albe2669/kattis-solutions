use std::{
    io::{self, Read, Write},
    process::Output,
};

// Fenwick tree
struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(n: usize) -> FenwickTree {
        FenwickTree {
            tree: vec![0; n + 1],
        }
    }

    fn sum(&self, mut i: usize) -> i32 {
        let mut s = 0;
        while i > 0 {
            s += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        s
    }

    fn update(&mut self, mut i: usize, x: i32) {
        while i <= self.tree.len() {
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

    let n: u32 = input.next().unwrap().parse().unwrap();
    let k: u32 = input.next().unwrap().parse().unwrap();
    let mut stuff = vec![false; n as usize + 1];

    let mut ft = FenwickTree::new(n as usize);

    for _ in 0..k {
        match input.next().unwrap() {
            "F" => {
                let i = input.next().unwrap().parse::<usize>().unwrap();
                stuff[i] = !stuff[i];
                if stuff[i] {
                    ft.update(i, 1);
                } else {
                    ft.update(i, -1);
                }
            }
            "C" => {
                let i = input.next().unwrap().parse::<usize>().unwrap();
                let j = input.next().unwrap().parse::<usize>().unwrap();
                output
                    .write_fmt(format_args!("{}\n", ft.sum(j) - ft.sum(i - 1)))
                    .unwrap();
            }
            _ => {}
        }
    }
}
