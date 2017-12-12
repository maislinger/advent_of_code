#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::{BTreeMap, BTreeSet};

struct GroupInfo {
    connections: BTreeMap<usize, Vec<usize>>,
    groups: BTreeMap<usize, Option<usize>>,
}

impl GroupInfo {
    fn new(connections: BTreeMap<usize, Vec<usize>>) -> GroupInfo {
        let groups = connections.keys().map(|k| (*k, None)).collect();
        GroupInfo {
            connections: connections,
            groups: groups,
        }
    }

    fn compute_group(&mut self, nr: usize) {
        let mut connected = BTreeSet::new();
        let mut to_check = BTreeSet::new();
        to_check.insert(nr);

        while !to_check.is_empty() {
            let subnr = *to_check.iter().nth(0).unwrap();
            connected.insert(subnr);
            let connected_to_nr = &self.connections[&subnr];
            for &nrc in connected_to_nr.iter() {
                if !connected.contains(&nrc) {
                    to_check.insert(nrc);
                }
            }
            to_check.remove(&subnr);
        }

        let group_identifier = *connected.iter().min().unwrap();
        for key in &connected {
            self.groups.insert(*key, Some(group_identifier));
        }
    }

    fn compute_all_groups(&mut self) {
        loop {
            let nr = self.groups.iter().map(|(a, b)| (*a, *b)).find(
                |x| x.1 == None,
            );

            match nr {
                Some((nr, _)) => self.compute_group(nr),
                None => break,
            };
        }
    }

    fn group_size(&self, nr: usize) -> usize {
        let group_nr = &self.groups[&nr];
        self.groups
            .iter()
            .filter(|&(_, &gnr)| *group_nr == gnr)
            .count()
    }

    fn total_groups(&self) -> usize {
        let mut g = BTreeSet::new();

        for value in self.groups.values() {
            g.insert(value.unwrap());
        }

        g.len()
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

fn convert_input(input: &str) -> BTreeMap<usize, Vec<usize>> {
    use regex::Regex;

    lazy_static! {
        static ref RE_LINE: Regex = Regex::new("([0-9]+) <-> ([0-9, ]+)").unwrap();
        static ref RE_NEIGHBORS: Regex = Regex::new("([0-9]+)").unwrap();
    }

    let mut result = BTreeMap::new();

    for cap in RE_LINE.captures_iter(input) {
        let program = cap.get(1).unwrap().as_str().to_owned().parse().unwrap();
        let neighbors_str = cap.get(2).unwrap().as_str();

        let neighbors = RE_NEIGHBORS
            .captures_iter(neighbors_str)
            .map(|cap| {
                cap.get(1).unwrap().as_str().to_owned().parse().unwrap()
            })
            .collect();
        result.insert(program, neighbors);
    }

    result
}

fn compute_solution_part_one(input: &str) -> usize {
    let connections = convert_input(input);
    let mut group_info = GroupInfo::new(connections);
    group_info.compute_group(0);
    group_info.group_size(0)
}

fn compute_solution_part_two(input: &str) -> usize {
    let connections = convert_input(input);
    let mut group_info = GroupInfo::new(connections);
    group_info.compute_all_groups();
    group_info.total_groups()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "0 <-> 2
                     1 <-> 1
                     2 <-> 0, 3, 4
                     3 <-> 2, 4
                     4 <-> 2, 3, 6
                     5 <-> 6
                     6 <-> 4, 5";

        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 6);

        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 2);
    }
}

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
