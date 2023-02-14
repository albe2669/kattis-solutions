use std::io;

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn fill(n: usize) -> Vec<u64> {
    let mut p2 = vec![0; n];
    p2[0] = 1;

    for i in 1..n {
        p2[i] = p2[i - 1] << 1;
    }

    p2
}

fn main() {
    let stdin = io::stdin();

    let n: usize = get_input(&stdin).trim().parse().unwrap();

    let fruits: Vec<u64> = get_input(&stdin)
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let p2 = fill(n);
    let mut ans = 0;

    (0..n).for_each(|i| {
        ans += fruits[i] * p2[n - 1];
    });

    (0..n).for_each(|i| {
        let mut w = fruits[i];
        if w < 200 {
            ans -= w;
        }

        (i + 1..n).for_each(|j| {
            w = fruits[i] + fruits[j];
            if w < 200 {
                ans -= w;
            }

            (j + 1..n).for_each(|k| {
                w = fruits[i] + fruits[j] + fruits[k];
                if w < 200 {
                    ans -= w;
                }
            });
        });
    });

    println!("{}", ans);
}
