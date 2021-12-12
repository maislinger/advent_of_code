fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d12 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let cavegraph = CaveGraph::from_str(input);
    let node = NodeList {
        id: cavegraph.id_start,
        prev: None,
    };

    compute_ways_no_double_small_cave(&node, &cavegraph)
}

fn compute_solution_part_two(input: &str) -> usize {
    let cavegraph = CaveGraph::from_str(input);
    let node = NodeList {
        id: cavegraph.id_start,
        prev: None,
    };

    compute_ways_one_double_small_cave(&node, &cavegraph)
}

fn compute_ways_no_double_small_cave(prev: &NodeList, cavegraph: &CaveGraph) -> usize {
    if prev.id == cavegraph.id_end {
        return 1;
    }

    let mut s = 0;

    for j in cavegraph.connections(prev.id) {
        if !(cavegraph.is_small_node(j) && prev.contains(j)) {
            let new_node = NodeList {
                id: j,
                prev: Some(prev),
            };
            s += compute_ways_no_double_small_cave(&new_node, cavegraph);
        }
    }

    s
}

fn compute_ways_one_double_small_cave(prev: &NodeList, cavegraph: &CaveGraph) -> usize {
    if prev.id == cavegraph.id_end {
        return 1;
    }

    let mut s = 0;

    for j in cavegraph.connections(prev.id) {
        if j == cavegraph.id_start {
            continue;
        }

        let new_node = NodeList {
            id: j,
            prev: Some(prev),
        };

        if cavegraph.is_small_node(j) && prev.contains(j) {
            s += compute_ways_no_double_small_cave(&new_node, cavegraph);
        } else {
            s += compute_ways_one_double_small_cave(&new_node, cavegraph);
        }
    }

    s
}

struct NodeList<'a> {
    id: usize,
    prev: Option<&'a NodeList<'a>>,
}

impl<'a> NodeList<'a> {
    fn contains(&self, id: usize) -> bool {
        if self.id == id {
            true
        } else if self.prev.is_some() {
            self.prev.unwrap().contains(id)
        } else {
            false
        }
    }
}

struct CaveGraph {
    n_small_nodes: usize,
    n_large_nodes: usize,
    id_start: usize,
    id_end: usize,
    data: Vec<usize>,
    names: Vec<String>,
}

impl CaveGraph {
    fn from_str(input: &str) -> Self {
        use std::cmp::Ordering;

        fn is_uppercase(s: &str) -> bool {
            s.to_uppercase() == s
        }

        let mut names = Vec::new();
        for line in input.lines() {
            for (i, s) in line.split('-').enumerate() {
                assert!(i < 2);
                names.push(s.to_owned());
            }
        }

        names.sort_by(|a, b| {
            let up_a = is_uppercase(a);
            let up_b = is_uppercase(b);

            if !up_a && up_b {
                Ordering::Less
            } else if up_a && !up_b {
                Ordering::Greater
            } else {
                a.cmp(b)
            }
        });
        names.dedup();

        let n_large_nodes = names.iter().filter(|s| is_uppercase(s)).count();
        let n_small_nodes = names.len() - n_large_nodes;

        let id_start = names.iter().position(|s| s.as_str() == "start").unwrap();
        let id_end = names.iter().position(|s| s.as_str() == "end").unwrap();

        let mut result = Self {
            n_small_nodes,
            n_large_nodes,
            id_start,
            id_end,
            data: vec![0; 2 * (n_large_nodes + n_small_nodes)],
            names,
        };

        for line in input.lines() {
            let mut line_iter = line.split('-');
            let name_a = line_iter.next().unwrap();
            let name_b = line_iter.next().unwrap();

            let id_a = result
                .names
                .iter()
                .position(|s| s.as_str() == name_a)
                .unwrap();
            let id_b = result
                .names
                .iter()
                .position(|s| s.as_str() == name_b)
                .unwrap();

            assert!(!(result.is_large_node(id_a) && result.is_large_node(id_b)));

            result.insert_edge(id_a, id_b);
        }

        result
    }

    fn insert_edge(&mut self, id_a: usize, id_b: usize) {
        self.insert_edge_raw(id_a, id_b);
        self.insert_edge_raw(id_b, id_a);
    }

    fn insert_edge_raw(&mut self, id_a: usize, id_b: usize) {
        let pos = self.data[2 * id_a];

        if pos == 0 {
            // No connections for id_a in data yet.
            self.data[2 * id_a] = self.data.len();
            self.data[2 * id_a + 1] = 1;
            self.data.push(id_b);
            return;
        }

        let n = self.data[2 * id_a + 1];

        for &id_c in &self.data[pos..(pos + n)] {
            if id_c == id_b {
                // Connection already there.
                return;
            }
        }

        self.data.insert(pos + n, id_b);
        self.data[2 * id_a + 1] += 1;

        // Update position of connections of other nodes.
        // They were shifted to the right.
        for other_pos in self
            .data
            .iter_mut()
            .step_by(2)
            .take(self.n_small_nodes + self.n_large_nodes)
            .filter(|p| **p > pos)
        {
            *other_pos += 1;
        }
    }

    fn is_small_node(&self, id: usize) -> bool {
        id < self.n_small_nodes
    }

    fn is_large_node(&self, id: usize) -> bool {
        id >= self.n_small_nodes
    }

    fn connections(&self, id: usize) -> ConnectionsIter {
        assert!(id < self.n_small_nodes + self.n_large_nodes);

        let pos = self.data[2 * id];

        if pos == 0 {
            return ConnectionsIter {
                nodes: &self.data[0..0],
                n: 0,
            };
        }

        let n_edges = self.data[2 * id + 1];

        ConnectionsIter {
            nodes: &self.data[pos..(pos + n_edges)],
            n: 0,
        }
    }
}

struct ConnectionsIter<'a> {
    nodes: &'a [usize],
    n: usize,
}

impl<'a> Iterator for ConnectionsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n >= self.nodes.len() {
            return None;
        }

        let result = Some(self.nodes[self.n]);
        self.n += 1;
        result
    }
}

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}
