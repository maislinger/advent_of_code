use std::collections::{BTreeMap, BTreeSet};
use std::ops::Add;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

struct NeighborIterator {
    origin: Coordinate,
    delta: Coordinate,
    return_none: bool,
    use_four_dimensions: bool,
}

impl NeighborIterator {
    fn new(origin: Coordinate, use_four_dimensions: bool) -> Self {
        let delta = if use_four_dimensions {
            Coordinate::new(1, 1, 1, 1)
        } else {
            Coordinate::new(1, 1, 1, 0)
        };
        Self {
            origin,
            delta,
            return_none: false,
            use_four_dimensions,
        }
    }

    fn step_delta(&mut self) {
        let vals = [
            &mut self.delta.x,
            &mut self.delta.y,
            &mut self.delta.z,
            &mut self.delta.w,
        ];
        let n_dim = if self.use_four_dimensions { 4 } else { 3 };
        for i in 0..n_dim {
            *vals[i] += 1;
            if *vals[i] == 2 {
                *vals[i] = -1;
            } else {
                break;
            }
        }

        if self.delta.is_zero() {
            self.step_delta();
        }

        let delta_w_ref = if self.use_four_dimensions { 1 } else { 0 };

        if self.delta.x == 1
            && self.delta.y == 1
            && self.delta.z == 1
            && self.delta.w == delta_w_ref
        {
            self.return_none = true;
        }
    }
}

impl Iterator for NeighborIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.return_none {
            None
        } else {
            self.step_delta();
            Some(self.origin + self.delta)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, Ord, Eq, PartialEq)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Coordinate {
    fn new(x: i64, y: i64, z: i64, w: i64) -> Self {
        Self { x, y, z, w }
    }

    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0 && self.w == 0
    }

    fn neighbors(&self, use_four_dimensions: bool) -> NeighborIterator {
        NeighborIterator::new(*self, use_four_dimensions)
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

fn parse_input(input: &str) -> BTreeSet<Coordinate> {
    let mut alive = BTreeSet::new();
    let z = 0;
    let w = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                alive.insert(Coordinate::new(x as i64, y as i64, z, w));
            }
        }
    }
    alive
}

fn step(alive: &BTreeSet<Coordinate>, use_four_dimensions: bool) -> BTreeSet<Coordinate> {
    let mut inactive = BTreeMap::new();
    let mut new_alive = BTreeSet::new();

    for c in alive.iter() {
        let mut alive_neighbors = 0;
        for cn in c.neighbors(use_four_dimensions) {
            if alive.contains(&cn) {
                alive_neighbors += 1;
            } else {
                *inactive.entry(cn).or_insert(0) += 1;
            }
        }
        if alive_neighbors == 2 || alive_neighbors == 3 {
            new_alive.insert(*c);
        }
    }

    for (c, _) in inactive.iter().filter(|(_, v)| **v == 3) {
        new_alive.insert(*c);
    }

    new_alive
}

fn test_reactor(use_four_dimensions: bool, input: &str) -> usize {
    let mut alive = parse_input(input);
    for _ in 0..6 {
        alive = step(&alive, use_four_dimensions);
    }
    alive.len()
}

fn compute_solution_part_one(input: &str) -> usize {
    test_reactor(false, input)
}

fn compute_solution_part_two(input: &str) -> usize {
    test_reactor(true, input)
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d17 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
