use std::io::{self, Read, Write};

struct Graph {
    edges: Vec<(usize, usize, usize)>,
    v: usize,
}

impl Graph {
    fn new(v: usize) -> Self {
        Graph {
            edges: Vec::with_capacity(v * v),
            v,
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, weight: usize) {
        self.edges.push((from, to, weight));
    }

    fn find(parent: &mut Vec<usize>, x: usize) -> usize {
        if parent[x] != x {
            parent[x] = Self::find(parent, parent[x]);
        }

        parent[x]
    }

    fn union(&self, parent: &mut Vec<usize>, rank: &mut [usize], x: usize, y: usize) {
        let x_root = Self::find(parent, x);
        let y_root = Self::find(parent, y);

        match rank[x_root].cmp(&rank[y_root]) {
            std::cmp::Ordering::Less => parent[x_root] = y_root,
            std::cmp::Ordering::Greater => parent[y_root] = x_root,
            std::cmp::Ordering::Equal => {
                parent[y_root] = x_root;
                rank[x_root] += 1;
            }
        }
    }

    fn kruskals_mst(&mut self) -> Vec<(usize, usize, usize)> {
        self.edges.sort_by(|(_,_,a), (_,_,b)| a.cmp(b));

        let mut parent: Vec<usize> = (0..self.v).collect();
        let mut rank = vec![0; self.v];

        let mut mst = vec![];

        for i in 0..self.edges.len() {
            let (from, to, weight) = self.edges[i];
            let x = Self::find(&mut parent, from);
            let y = Self::find(&mut parent, to);

            if x != y {
                mst.push((from, to, weight));
                self.union(&mut parent, &mut rank, x, y);
            }
        }

        mst
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().lock().read_to_string(&mut buffer).unwrap();
    let mut input = buffer.split_whitespace().map(|x| x.trim().parse::<usize>().unwrap());

    let stdout = io::stdout().lock();
    let mut output = io::BufWriter::new(stdout);

    let n = input.next().unwrap();

    let mut graph = Graph::new(n);

    for i in 0..n {
        for j in 0..n {
            if j <= i {
                input.next();
                continue;
            }

            let x = input.next().unwrap();
            if x != 0 {
                graph.add_edge(i, j, x);
            }
        }
    }

    let mst = graph.kruskals_mst();
    for (from, to, _) in mst {
        output.write_fmt(format_args!("{} {}\n", from + 1, to + 1)).unwrap();
    }

    output.flush().unwrap();
}
