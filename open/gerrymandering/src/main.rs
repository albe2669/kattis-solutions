use std::io::{self};
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

fn get_input(io : &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

fn main() {
    let stdin = io::stdin();

    let s : Vec<i64> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

    let mut map : BTreeMap<i64, (i64, i64)> = BTreeMap::new();
    let mut aw : i64 = 0;
    let mut bw : i64 = 0;
    let mut v = 0;
    
    for _i in 0..s[0]  {
        let t : Vec<i64> = get_input(&stdin).trim().split_whitespace().map(|n| n.parse().unwrap()).collect();
        v += t[1] + t[2];

        match map.entry(t[0]) {
            Entry::Occupied(o) => {
                let entry = o.into_mut();
                *entry = (entry.0 + t[1],entry.1 + t[2]);
            }
            Entry::Vacant(_v) => {
                map.insert(t[0], (t[1], t[2]));
            },
        }
    }

    for (_i, votes) in map {
        let winning_vote : i64 = (votes.0 + votes.1)/2 + 1;

        if (votes.0 > votes.1) {
            aw += votes.0 - winning_vote;
            bw += votes.1;

            println!("A {} {}", votes.0 - winning_vote, votes.1);
        } else {
            bw += votes.1 - winning_vote;
            aw += votes.0;

            println!("B {} {}", votes.0, votes.1 - winning_vote);
        }
    }

    println!("{:?}", ((aw - bw).abs() as f64) / (v as f64)) ;
}