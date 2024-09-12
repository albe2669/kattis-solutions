use std::{collections::BTreeSet, io::{self, Read, Write}, ops::Bound::{Excluded, Unbounded}};

type Store = (i64, i64);

fn get_elem(set: &BTreeSet<Store>, h: i64) -> Option<Store> {
    set.range((Unbounded, Excluded((h, -1)))).next_back().copied()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace().map(|s| s.trim().parse::<i64>().unwrap());

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n = input.next().unwrap();
    let mut stores = Vec::with_capacity(n as usize);

    for _ in 0..n {
        let store = (
            input.next().unwrap(),
            input.next().unwrap()
        );

        stores.push(store);
    }

    stores.sort();

    let mut intervals: BTreeSet<Store> = BTreeSet::new();
    intervals.insert((0, i64::MAX));

    let mut z = 0;
    for store in stores {
        let mut t = store.0;
        let h = store.1;

        while t > 0 {
            let old_point = get_elem(&intervals, h);

            if old_point.is_none() {
                break;
            }

            let (old_t, old_h) = old_point.unwrap();
            if old_h > h {
                intervals.remove(&old_point.unwrap());
                intervals.insert((old_t, h));
                intervals.insert((h, old_h));
                continue;
            }

            if old_h - old_t > t {
                intervals.remove(&old_point.unwrap());
                intervals.insert((old_t, old_h - t));
                t = 0;
                continue;
            }

            t -= old_h - old_t;
            intervals.remove(&old_point.unwrap());
        }

        if t > 0 {
            intervals.insert((h - store.0+t,h));
            continue;
        }

        z += 1;
    }

    output.write_fmt(format_args!("{}\n", z)).unwrap();
}
