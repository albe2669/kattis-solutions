use std::io::{self, Read, Write};

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split('\n');

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let mut input_vecs = input.next().unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap());
    let _s = input_vecs.next().unwrap();
    let c = input_vecs.next().unwrap();
    let k = input_vecs.next().unwrap();

    let mut socks: Vec<u64> = input.next().unwrap().split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    socks.sort();

    let mut washing_machines = 1;
    let mut curr_machine = 1;
    let mut curr_lowest_color = socks[0];

    (1..socks.len()).for_each(|i| {
        if curr_machine == c || socks[i] - curr_lowest_color > k {
            washing_machines += 1;
            curr_machine = 0;
            curr_lowest_color = socks[i];
        }

        curr_machine += 1;
    });

    output.write_fmt(format_args!("{}\n", washing_machines)).unwrap();
}
