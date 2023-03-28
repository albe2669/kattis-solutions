use std::io::{self, Read, Write};

type Value = u32;

#[derive(Default, Debug)]
struct Node {
    children: [Option<Box<Node>>; 26],
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
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let (ns, rest) = buffer.split_once('\n').unwrap();

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n: u32 = ns.trim().parse().unwrap();
    let mut chars = rest.chars();

    let mut trie = Trie::new();
    for _ in 0..n {
        let mut node = &mut trie.root;

        for nchar in chars.by_ref() {
            if nchar == '\n' {
                break;
            }

            match node.children[nchar as usize - 97] {
                Some(ref mut n) => node = n,
                None => {
                    node.children[nchar as usize - 97] = Some(Box::default());
                    node = node.children[nchar as usize - 97].as_mut().unwrap();
                }
            }

            node.value += 1;
        }

        writeln!(output, "{}", node.value - 1).unwrap();
    }
}
