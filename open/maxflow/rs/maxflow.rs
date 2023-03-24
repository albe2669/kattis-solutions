use std::{
    collections::{HashMap, VecDeque},
    io,
};

fn get_input(io: &std::io::Stdin) -> String {
    let mut buffer = String::new();
    io.read_line(&mut buffer).expect("Failed");
    buffer
}

type Index = usize;

pub struct Graph {
    edges: HashMap<Index, HashMap<Index, (i32, i32)>>,
    counter: Index,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
            counter: 0,
        }
    }

    fn neighbors(&self, node: Index) -> Vec<(Index, Index, (i32, i32))> {
        match self.edges.get(&node) {
            Some(e) => e.iter().map(|(to, edge)| (node, *to, *edge)).collect(),
            None => Vec::new(),
        }
    }

    fn add_edge(&mut self, from: Index, to: Index, capacity: i32) {
        let mut list = self.edges.entry(from).or_insert_with(HashMap::new);
        list.insert(to, (capacity, 0));

        // Maybe remove?
        list = self.edges.entry(to).or_insert_with(HashMap::new);
        list.insert(from, (0, 0));
    }

    fn get_mut_edge(&mut self, from: Index, to: Index) -> &mut (i32, i32) {
        let list = self.edges.get_mut(&from).unwrap();
        list.get_mut(&to).unwrap()
    }
}

struct Solver<'a> {
    graph: &'a mut Graph,
    parent: Vec<Index>,
    source: Index,
    sink: Index,
}

impl<'a> Solver<'a> {
    fn new(graph: &'a mut Graph, source: Index, sink: Index) -> Self {
        let n = graph.edges.len();
        Self {
            graph,
            parent: vec![0; n],
            source,
            sink,
        }
    }

    fn bfs(&mut self, node: Index) -> bool {
        let mut visited = vec![false; self.graph.edges.len()];
        let mut queue = VecDeque::new();
        queue.push_back(node);

        self.parent[node] = usize::MAX;
        visited[node] = true;

        while let Some(node) = queue.pop_front() {
            for (from, to, (capacity, flow)) in self.graph.neighbors(node) {
                if !visited[to] && capacity > 0 {
                    self.parent[to] = from;
                    visited[to] = true;
                    queue.push_back(to);

                    if to == self.sink {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn solve(&mut self) -> i32 {
        let mut maxflow = 0;

        while self.bfs(self.source) {
            let mut flow = std::i32::MAX;

            let mut node = self.sink;
            while node != self.source {
                let p = self.parent[node];
                let edge = self.graph.get_mut_edge(p, node);
                flow = std::cmp::min(flow, edge.0);
                node = p;
            }

            node = self.sink;
            while node != self.source {
                let p = self.parent[node];

                let mut edge = self.graph.get_mut_edge(node, p);
                edge.0 += flow;
                edge.1 -= flow;

                edge = self.graph.get_mut_edge(p, node);
                edge.0 -= flow;
                edge.1 += flow;

                node = p;
            }

            maxflow += flow;
        }

        maxflow
    }

    fn get_edges(&self) -> Vec<(Index, Index, (i32, i32))> {
        let mut edges = Vec::new();
        for (from, list) in self.graph.edges.iter() {
            for (to, edge) in list.iter() {
                if edge.1 > 0 {
                    edges.push((*from, *to, *edge));
                }
            }
        }

        edges
    }

    fn print(&self) {
        for (from, list) in self.graph.edges.iter() {
            println!("{} -> {:?}", from, list);
        }
    }
}

fn main() {
    let stdin = io::stdin();

    let nmst: Vec<u32> = get_input(&stdin)
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let n = nmst[0] as usize; // nodes
    let m = nmst[1] as usize; // edges
    let s = nmst[2]; // source
    let t = nmst[3]; // sink / target

    let mut graph = Graph::new();

    for _ in 0..m {
        let uvc: Vec<u32> = get_input(&stdin)
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let u = uvc[0] as usize; // from
        let v = uvc[1] as usize; // to
        let c = uvc[2]; // capacity

        graph.add_edge(u, v, c as i32);
    }

    let mut solver = Solver::new(&mut graph, s as usize, t as usize);
    let t = solver.solve();
    let mut edges = solver.get_edges();
    edges.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => a.1.cmp(&b.1),
        x => x,
    });

    println!("{} {} {}", n, t, edges.len());
    for (from, to, (capacity, flow)) in edges {
        println!("{} {} {}", from, to, flow);
    }
}
