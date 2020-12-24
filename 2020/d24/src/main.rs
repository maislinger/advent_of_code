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

#[derive(Copy, Clone)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::East,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Coordinate {
    q: i64,
    r: i64,
}

impl Coordinate {
    fn new(q: i64, r: i64) -> Self {
        Self { q, r }
    }

    fn to_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::East => Self::new(self.q + 1, self.r),
            Direction::SouthEast => Self::new(self.q, self.r + 1),
            Direction::SouthWest => Self::new(self.q - 1, self.r + 1),
            Direction::West => Self::new(self.q - 1, self.r),
            Direction::NorthWest => Self::new(self.q, self.r - 1),
            Direction::NorthEast => Self::new(self.q + 1, self.r - 1),
        }
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        Direction::iter().map(move |d| self.to_direction(d))
    }

    fn parse_line(line: &str) -> Self {
        let mut result = Self::new(0, 0);
        let mut north = false;
        let mut south = false;
        for c in line.chars() {
            match (north, south, c) {
                (false, false, 'n') => {
                    north = true;
                }
                (false, false, 's') => {
                    south = true;
                }
                (false, false, 'e') => {
                    result = result.to_direction(Direction::East);
                }
                (true, false, 'e') => {
                    result = result.to_direction(Direction::NorthEast);
                    north = false;
                }
                (false, true, 'e') => {
                    result = result.to_direction(Direction::SouthEast);
                    south = false;
                }
                (false, false, 'w') => {
                    result = result.to_direction(Direction::West);
                }
                (true, false, 'w') => {
                    result = result.to_direction(Direction::NorthWest);
                    north = false;
                }
                (false, true, 'w') => {
                    result = result.to_direction(Direction::SouthWest);
                    south = false;
                }
                _ => panic!("invalid input"),
            }
        }
        result
    }
}

fn parse_input(input: &str) -> BTreeSet<Coordinate> {
    let mut black = BTreeSet::new();
    for line in input.lines() {
        let coordinate = Coordinate::parse_line(line);
        if black.contains(&coordinate) {
            black.remove(&coordinate);
        } else {
            black.insert(coordinate);
        }
    }
    black
}

fn exhibit_step(black: &BTreeSet<Coordinate>) -> BTreeSet<Coordinate> {
    let mut next_black = BTreeSet::new();
    let mut white_neighbors: BTreeMap<Coordinate, usize> = BTreeMap::new();

    for &c in black.iter() {
        let mut count = 0;
        for neighbor in c.neighbors() {
            if black.contains(&neighbor) {
                count += 1;
            } else {
                white_neighbors
                    .entry(neighbor)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
        if !(count == 0 || count > 2) {
            next_black.insert(c);
        }
    }

    for (&c, _) in white_neighbors.iter().filter(|(_, &count)| count == 2) {
        next_black.insert(c);
    }

    next_black
}

fn compute_solution_part_one(input: &str) -> usize {
    let black = parse_input(input);
    black.len()
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut black = parse_input(input);
    for _ in 0..100 {
        black = exhibit_step(&black);
    }
    black.len()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d24 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
