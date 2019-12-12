extern crate regex;

use std::collections::{BTreeMap, BTreeSet};
use regex::Regex;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z, vx: 0, vy: 0, vz: 0 }
    }

    fn update_vi(vi: &mut i64, i: i64, other_i: i64) {
        if other_i > i {
            *vi += 1;
        } else if other_i < i {
            *vi -= 1;
        }
    }

    fn update_v(&mut self, other: &Self) {
        Self::update_vi(&mut self.vx, self.x, other.x);
        Self::update_vi(&mut self.vy, self.y, other.y);
        Self::update_vi(&mut self.vz, self.z, other.z);
    }

    fn update_pos(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn kinetic_energy(&self) -> i64 {
        self.vx.abs() + self.vy.abs() + self.vz.abs()
    }

    fn potential_energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn parse_input(input: &str) -> Vec<Moon> {
    let re = Regex::new(r"x=(-?\d*), y=(-?\d*), z=(-?\d*)").unwrap();
    re.captures_iter(input)
    .map(|c| {
        let x = c[1].parse().unwrap();
        let y = c[2].parse().unwrap();
        let z = c[3].parse().unwrap();
        Moon::new(x, y, z)
    })
    .collect()
}

fn update_v(moons: &mut Vec<Moon>) {
    for i in 1..moons.len() {
        let (left, right) = moons.split_at_mut(i);
        let mi = left.last_mut().unwrap();
        for mj in right.iter_mut() {
            mi.update_v(&mj);
            mj.update_v(&mi);
        }
    }
}

fn update_pos(moons: &mut Vec<Moon>) {
    for m in moons.iter_mut() {
        m.update_pos();
    }
}

fn compute_solution_part_one(input: &str) -> i64 {
    let mut moons = parse_input(input);
    for _ in 0..1000 {
        update_v(&mut moons);
        update_pos(&mut moons);
    }
    moons.iter().map(|m| m.total_energy()).sum()
}

fn compute_solution_part_two(input: &str) -> i64 {
    0
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
