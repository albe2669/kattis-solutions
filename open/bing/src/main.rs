use std::{
    collections::HashMap,
    hash::Hash,
    io::{self, Read, Write},
    ops::Add,
};

type Input = String;
type Key = char;
type Value = u32;

#[derive(Default, Debug)]
struct Node {
    children: HashMap<Key, Node>,
    value: Value,
}

#[derive(Debug, Default)]
pub struct Trie {
    root: Node,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, value: Input) -> Value {
        let mut node = &mut self.root;
        for c in value.chars() {
            node = node.children.entry(c).or_default();
            node.value += 1; // Added as a modification
        }

        node.value - 1 // Added as a modification
    }

    pub fn get(&self, key: Input) -> Option<&Value> {
        let mut node = &self.root;
        for c in key.chars() {
            if node.children.contains_key(&c) {
                node = node.children.get(&c).unwrap();
            } else {
                return None;
            }
        }

        Some(&node.value)
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: u32 = input.next().unwrap().trim().parse().unwrap();

    let mut trie = Trie::new();
    for _ in 0..n {
        let instring = input.next().unwrap().trim().to_string();
        let value = trie.insert(instring);
        writeln!(output, "{}", value).unwrap();
    }
}
