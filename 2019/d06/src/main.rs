use std::collections::BTreeMap;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

struct OrbitalBody {
    name: String,
    children: Vec<OrbitalBody>,
}

impl OrbitalBody {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            children: Vec::new(),
        }
    }

    fn insert(&mut self, child: Self) {
        self.children.push(child);
    }

    fn get(&mut self, name: &str) -> Option<&mut Self> {
        if self.name == name {
            Some(self)
        } else {
            self.children.iter_mut().filter_map(|c| c.get(name)).next()
        }
    }

    fn count_orbits(&self, level: u64) -> u64 {
        level
            + self
                .children
                .iter()
                .map(|c| c.count_orbits(level + 1))
                .sum::<u64>()
    }

    fn measure_distance(&self, a: &str, b: &str) -> (Option<u64>, Option<u64>) {
        let mut result_a = if self.name == a { Some(0) } else { None };
        let mut result_b = if self.name == b { Some(0) } else { None };

        for c in &self.children {
            let (child_a, child_b) = c.measure_distance(a, b);
            if child_a.is_some() {
                result_a = child_a;
            }
            if child_b.is_some() {
                result_b = child_b;
            }
        }

        match (result_a, result_b) {
            (None, None) => (None, None),
            (Some(i), None) => (Some(i + 1), None),
            (None, Some(i)) => (None, Some(i + 1)),
            (Some(i), Some(j)) => (Some(i), Some(j)),
        }
    }
}

fn parse_input(input: &str) -> OrbitalBody {
    let mut orbits = BTreeMap::new();
    for line in input.lines() {
        let b: Vec<_> = line.split(')').collect();
        let children = orbits.entry(b[0].to_owned()).or_insert_with(Vec::new);
        children.push(b[1].to_owned());
    }
    let mut root = OrbitalBody::new("COM");
    let mut leaves = vec!["COM".to_owned()];

    while !leaves.is_empty() {
        let parent_name = leaves.pop().unwrap();
        let parent = root.get(&parent_name).unwrap();
        let child_names = orbits.entry(parent_name).or_insert_with(Vec::new);
        for child_name in child_names {
            let child = OrbitalBody::new(child_name);
            parent.insert(child);
            leaves.push((*child_name).to_string());
        }
    }

    root
}

fn compute_solution_part_one(input: &str) -> u64 {
    let root = parse_input(input);
    root.count_orbits(0)
}

fn compute_solution_part_two(input: &str) -> u64 {
    let root = parse_input(input);
    let (a, b) = root.measure_distance("YOU", "SAN");
    let a = a.unwrap();
    let b = b.unwrap();
    a + b - 2
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d06 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
