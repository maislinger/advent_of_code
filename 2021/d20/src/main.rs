use std::collections::{BTreeMap, BTreeSet};

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d20 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut image = Image::from_str(input);
    for _ in 0..2 {
        image.step();
    }
    image.count_lit_pixels()
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut image = Image::from_str(input);
    for _ in 0..50 {
        image.step();
    }
    image.count_lit_pixels()
}

#[derive(Debug)]
struct Image {
    enhancement_rule: Vec<bool>,
    default: bool,
    pixels: BTreeMap<(i64, i64), bool>,
}

impl Image {
    fn from_str(input: &str) -> Self {
        fn char_to_bool(c: char) -> bool {
            match c {
                '#' => true,
                '.' => false,
                _ => panic!("invalid input"),
            }
        }

        let input = input.trim();
        let enhancement_rule: Vec<bool> = input
            .lines()
            .next()
            .unwrap()
            .trim()
            .chars()
            .map(char_to_bool)
            .collect();
        assert_eq!(512, enhancement_rule.len());

        let mut pixels = BTreeMap::new();

        for (i, line) in input.lines().skip(2).enumerate() {
            let line = line.trim();
            for (j, c) in line.chars().enumerate() {
                pixels.insert((i as i64, j as i64), char_to_bool(c));
            }
        }

        Self {
            enhancement_rule,
            default: false,
            pixels,
        }
    }

    fn get(&self, i: i64, j: i64) -> bool {
        if let Some(b) = self.pixels.get(&(i, j)) {
            *b
        } else {
            self.default
        }
    }

    fn step(&mut self) {
        let new_default = if self.default {
            *self.enhancement_rule.last().unwrap()
        } else {
            *self.enhancement_rule.first().unwrap()
        };

        let mut new_pixels = BTreeMap::new();

        let mut todo = BTreeSet::new();
        for &(i, j) in self.pixels.keys() {
            for (ni, nj) in square(i, j) {
                todo.insert((ni, nj));
            }
        }

        for &(i, j) in todo.iter() {
            let mut index_builder = IndexBuilder::new();

            for (ni, nj) in square(i, j) {
                let v = self.get(ni, nj);
                index_builder.add(v);
            }

            let index = index_builder.finalize();
            new_pixels.insert((i, j), self.enhancement_rule[index]);
        }

        self.default = new_default;
        self.pixels = new_pixels;

        self.cleanup();
    }

    fn cleanup(&mut self) {
        let mut todo = BTreeSet::new();
        for &(i, j) in self.pixels.keys() {
            let mut all_default = true;
            for (ni, nj) in square(i, j) {
                all_default = all_default && self.get(ni, nj) == self.default;
                if !all_default {
                    break;
                }
            }
            if all_default {
                todo.insert((i, j));
            }
        }

        for k in todo.iter() {
            self.pixels.remove(k);
        }
    }

    fn count_lit_pixels(&self) -> usize {
        self.pixels.values().map(|p| if *p { 1 } else { 0 }).sum()
    }
}

struct IndexBuilder {
    count: usize,
    result: usize,
}

impl IndexBuilder {
    fn new() -> Self {
        Self {
            count: 0,
            result: 0,
        }
    }

    fn add(&mut self, new_val: bool) {
        assert!(self.count < 9);
        self.result <<= 1;
        if new_val {
            self.result += 1;
        }
        self.count += 1;
    }

    fn finalize(&self) -> usize {
        assert_eq!(self.count, 9);
        self.result
    }
}

fn square(i: i64, j: i64) -> SquareIter {
    SquareIter { dirnum: 0, i, j }
}

struct SquareIter {
    dirnum: usize,
    i: i64,
    j: i64,
}

impl Iterator for SquareIter {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        let (di, dj) = match self.dirnum {
            0 => Some((-1, -1)),
            1 => Some((-1, 0)),
            2 => Some((-1, 1)),
            3 => Some((0, -1)),
            4 => Some((0, 0)),
            5 => Some((0, 1)),
            6 => Some((1, -1)),
            7 => Some((1, 0)),
            8 => Some((1, 1)),
            _ => None,
        }?;

        self.dirnum += 1;

        Some((self.i + di, self.j + dj))
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
