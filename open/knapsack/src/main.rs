use std::cmp::max;
use std::io::{self, BufRead};

fn solve(values: Vec<usize>, weights: Vec<usize>, n: usize, capacity: usize) {
    let mut knapsack = vec![vec![0; n + 1]; capacity + 1];

    for i in 1..n {
        for w in 1..capacity {
            if w < weights[i] {
                knapsack[i][w] = knapsack[i - 1][w];
                continue;
            }

            knapsack[i][w] = max(
                values[i] + knapsack[i - 1][w - weights[i]],
                knapsack[i - 1][w],
            );
        }
    }

    let mut res = knapsack[n][capacity];
    let mut items = Vec::new();

    let mut weight = capacity;
    let mut i = n;

    while i > 0 && res > 0 {
        if res != knapsack[i - 1][weight] {
            items.push(i.to_string());
            res -= values[i - 1];
            weight -= weights[i - 1];
        }
        i -= 1;
    }

    println!("{}", items.len());
    println!("{}", items.join(" "));
}

fn main() {
    let mut stdin = io::stdin().lock().lines();

    while let Some(line) = stdin.next() {
        let cn = line
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let capacity = cn[0];
        let n = cn[1];

        let mut weights = vec![0, n + 1];
        let mut values = vec![0, n + 1];

        for i in 1..n {
            let obj: Vec<usize> = stdin
                .next()
                .unwrap()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            values[i] = obj[0];
            weights[i] = obj[1];
        }

        solve(values, weights, n, capacity);
    }
}
