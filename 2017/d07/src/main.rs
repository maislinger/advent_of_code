#[macro_use]
extern crate lazy_static;
extern crate regex;

struct Node {
    name: String,
    weight: i64,
    total_weight: Option<i64>,
    balanced_weight: bool,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn new(name: String, weight: i64) -> Node {
        Node {
            name: name,
            weight: weight,
            total_weight: None,
            balanced_weight: false,
            parent: None,
            children: Vec::new(),
        }
    }
}

struct Tree {
    root: usize,
    nodes: Vec<Node>,
}

impl Tree {
    fn new(nodes: Vec<Node>) -> Tree {
        let mut res = Tree { root: 0, nodes };
        res.compute_root();
        res
    }

    fn compute_root(&mut self) {
        while let Some(id) = self.nodes[self.root].parent {
            self.root = id;
        }
    }

    fn compute_weights_node(&mut self, id: usize) {
        let (sum, _, all_same) = self.nodes[id]
            .children
            .iter()
            .map(|id| self.nodes[*id].total_weight.unwrap())
            .fold(
                (self.nodes[id].weight, None, true),
                |(s, vp, all_same): (i64, Option<i64>, bool), v| match vp {
                    Some(vp) => (s + v, Some(v), all_same && v == vp),
                    None => (s + v, Some(v), true),
                },
            );
        self.nodes[id].total_weight = Some(sum);
        self.nodes[id].balanced_weight = all_same;
    }

    fn find_unfinished_child(&self, id: usize) -> Option<usize> {
        self.nodes[id]
            .children
            .iter()
            .filter(|id| match self.nodes[**id].total_weight {
                Some(_) => false,
                None => true,
            })
            .cloned()
            .nth(0)
    }

    fn compute_total_weight(&mut self) {
        let mut id = self.root;

        loop {
            let unfinished_child = self.find_unfinished_child(id);
            match unfinished_child {
                Some(child_id) => {
                    id = child_id;
                }
                None => {
                    self.compute_weights_node(id);
                    if let Some(parent_id) = self.nodes[id].parent {
                        id = parent_id;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    fn find_unbalanced_child(&self, id: usize) -> Option<usize> {
        self.nodes[id]
            .children
            .iter()
            .filter(|id| !self.nodes[**id].balanced_weight)
            .cloned()
            .nth(0)
    }

    fn find_unbalanced_root(&self) -> usize {
        let mut id = self.root;
        loop {
            let unbalanced_child = self.find_unbalanced_child(id);
            match unbalanced_child {
                Some(child_id) => {
                    id = child_id;
                }
                None => {
                    break;
                }
            }
        }
        id
    }
}

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );
    contents.trim().to_owned()
}

fn convert_input(input: &str) -> Tree {
    use regex::Regex;
    use std::collections::BTreeMap;

    lazy_static! {
        static ref RE_NODE: Regex = Regex::new(r"([a-z]+) \(([0-9]+)\)(?: -> (.*))?").unwrap();
        static ref RE_CHILDREN: Regex = Regex::new(r"([a-z]+)").unwrap();
    }

    let mut nodes = Vec::new();
    let mut indices = BTreeMap::new();

    for (i, cap) in RE_NODE.captures_iter(input).enumerate() {
        let name = cap.get(1).unwrap().as_str();
        let weight = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
        nodes.push(Node::new(name.to_owned(), weight));
        indices.insert(name.to_owned(), i);
    }

    for (i, cap) in RE_NODE.captures_iter(input).enumerate() {
        if let Some(children) = cap.get(3) {
            let children = children.as_str();
            for cap_children in RE_CHILDREN.captures_iter(children) {
                cap_children
                    .iter()
                    .skip(1)
                    .map(|c| {
                        let c = c.unwrap().as_str();
                        let child_index = indices[c];
                        nodes[i].children.push(child_index);
                        nodes[child_index].parent = Some(i);
                    })
                    .count();
            }
        }
    }

    Tree::new(nodes)
}

fn compute_solution_part_one(input: &str) -> String {
    let tree = convert_input(input);
    let id = tree.root;
    tree.nodes[id].name.clone()
}

fn compute_solution_part_two(input: &str) -> i64 {
    let mut tree = convert_input(input);
    tree.compute_total_weight();
    let id = tree.find_unbalanced_root();
    let nr_c = tree.nodes[id].children.len();
    let total_weights: Vec<_> = tree.nodes[id]
        .children
        .iter()
        .map(|id| tree.nodes[*id].total_weight.unwrap())
        .collect();
    let index = total_weights
        .iter()
        .zip(total_weights.iter().cycle().skip(1))
        .zip(total_weights.iter().cycle().skip(nr_c - 1))
        .map(|((a, b), c)| a == b || a == c)
        .enumerate()
        .find(|&(_, b)| !b)
        .unwrap()
        .0;

    let correct_weight = if index == 0 {
        total_weights[1]
    } else {
        total_weights[0]
    };

    let id = tree.nodes[id].children[index];
    let diff = correct_weight - tree.nodes[id].total_weight.unwrap();
    tree.nodes[id].weight + diff
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "pbga (66)
                     xhth (57)
                     ebii (61)
                     havc (66)
                     ktlj (57)
                     fwft (72) -> ktlj, cntj, xhth
                     qoyq (66)
                     padx (45) -> pbga, havc, qoyq
                     tknk (41) -> ugml, padx, fwft
                     jptl (61)
                     ugml (68) -> gyxo, ebii, jptl
                     gyxo (61)
                     cntj (57)"
            .to_owned();

        let solution = compute_solution_part_one(&input);
        assert_eq!(solution, "tknk".to_owned());
        let solution = compute_solution_part_two(&input);
        assert_eq!(solution, 60);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d07 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
