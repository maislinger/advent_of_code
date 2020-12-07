use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(Debug)]
struct BagInfo {
    id_from_name: BTreeMap<String, usize>,
    name_from_id: BTreeMap<usize, String>,
    parents: BTreeMap<usize, Vec<usize>>,
    children: BTreeMap<usize, Vec<(usize, usize)>>,
}

impl BagInfo {
    fn new(
        id_from_name: BTreeMap<String, usize>,
        name_from_id: BTreeMap<usize, String>,
        parents: BTreeMap<usize, Vec<usize>>,
        children: BTreeMap<usize, Vec<(usize, usize)>>,
    ) -> Self {
        Self {
            id_from_name,
            name_from_id,
            parents,
            children,
        }
    }

    fn number_of_hulls(&self, main_id: usize) -> usize {
        let mut todo = vec![main_id];
        let mut done = BTreeSet::new();
        let mut result = 0;
        while !todo.is_empty() {
            let id = todo.pop().unwrap();
            if done.contains(&id) {
                continue;
            }
            for parent in self.parents.get(&id).unwrap().iter() {
                if !done.contains(parent) {
                    todo.push(*parent);
                }
            }
            done.insert(id);
            if id != main_id {
                result += 1;
            }
        }
        result
    }

    fn total_bags_within(&self, id: usize) -> usize {
        let mut result = 1;
        for &(n, child_id) in self.children.get(&id).unwrap().iter() {
            result += n * self.bags_within(child_id) + n;
        }
        result
    }

    fn bags_within(&self, id: usize) -> usize {
        self.total_bags_within(id) - 1
    }
}

fn parse_input(input: &str) -> BagInfo {
    let re_parent = Regex::new(r"(.*?) bags contain").unwrap();
    let re_children = Regex::new(r"(\d+?) (.*?) bag").unwrap();
    //let re_no_children = Regex::new(r"no other bags").unwrap();

    let mut next_id = 0;
    let mut id_from_name = BTreeMap::new();
    let mut name_from_id = BTreeMap::new();
    let mut parents = BTreeMap::new();
    let mut children = BTreeMap::new();

    let mut add_name = |s: String| {
        if !id_from_name.contains_key(&s) {
            id_from_name.insert(s.clone(), next_id);
            name_from_id.insert(next_id, s);
            next_id += 1;
            next_id - 1
        } else {
            *id_from_name.get(&s).unwrap()
        }
    };

    for line in input.lines() {
        let parent = re_parent.captures_iter(line).next().unwrap()[1].to_owned();
        let parent_id = add_name(parent);
        children.entry(parent_id).or_insert_with(Vec::new);
        parents.entry(parent_id).or_insert_with(Vec::new);

        if line.contains("no other bags") {
            continue;
        }
        for cap in re_children.captures_iter(line) {
            let n = cap[1].parse().unwrap();
            let child = cap[2].to_owned();
            let child_id = add_name(child);
            children.entry(child_id).or_insert_with(Vec::new);
            parents.entry(child_id).or_insert_with(Vec::new);
            parents.get_mut(&child_id).unwrap().push(parent_id);
            children.get_mut(&parent_id).unwrap().push((n, child_id));
        }
    }

    BagInfo::new(id_from_name, name_from_id, parents, children)
}

fn compute_solution_part_one(input: &str) -> usize {
    let bag_info = parse_input(input);
    let id = bag_info.id_from_name.get("shiny gold").unwrap();
    bag_info.number_of_hulls(*id)
}

fn compute_solution_part_two(input: &str) -> usize {
    let bag_info = parse_input(input);
    let id = bag_info.id_from_name.get("shiny gold").unwrap();
    bag_info.bags_within(*id)
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
