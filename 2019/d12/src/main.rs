extern crate regex;

use regex::Regex;
use std::cmp::Ordering;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct PhaseSpace {
    x: [i64; 4],
    v: [i64; 4],
}

impl PhaseSpace {
    fn from_vec(x: Vec<i64>) -> Self {
        Self {
            x: [x[0], x[1], x[2], x[3]],
            v: [0; 4],
        }
    }

    fn update_v(&mut self) {
        for i in 0..4 {
            for j in (i + 1)..4 {
                let delta = match self.x[i].cmp(&self.x[j]) {
                    Ordering::Less => 1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                };
                self.v[i] += delta;
                self.v[j] -= delta;
            }
        }
    }

    fn update_x(&mut self) {
        for i in 0..4 {
            self.x[i] += self.v[i];
        }
    }

    fn find_period(&mut self) -> u64 {
        let mut step = 0;
        let init = self.clone();

        loop {
            self.update_v();
            self.update_x();
            step += 1;
            if *self == init {
                break;
            }
        }

        step
    }
}

fn parse_input(input: &str) -> [PhaseSpace; 3] {
    let re = Regex::new(r"x=(-?\d*), y=(-?\d*), z=(-?\d*)").unwrap();
    let mut x = Vec::new();
    let mut y = Vec::new();
    let mut z = Vec::new();
    for c in re.captures_iter(input) {
        x.push(c[1].parse().unwrap());
        y.push(c[2].parse().unwrap());
        z.push(c[3].parse().unwrap());
    }
    [
        PhaseSpace::from_vec(x),
        PhaseSpace::from_vec(y),
        PhaseSpace::from_vec(z),
    ]
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn compute_solution_part_one(input: &str) -> i64 {
    let mut phase_spaces = parse_input(input);
    for _ in 0..1000 {
        for phase_space in &mut phase_spaces {
            phase_space.update_v();
            phase_space.update_x();
        }
    }

    let mut total_energy = 0;

    for i in 0..4 {
        let mut kinetic_energy = 0;
        let mut potential_energy = 0;
        for phase_space in &phase_spaces {
            kinetic_energy += phase_space.x[i].abs();
            potential_energy += phase_space.v[i].abs();
        }
        total_energy += kinetic_energy * potential_energy;
    }

    total_energy
}

fn compute_solution_part_two(input: &str) -> u64 {
    let mut phase_spaces = parse_input(input);
    let mut periods = [0; 3];
    for i in 0..3 {
        let period = phase_spaces[i].find_period();
        periods[i] = period;
    }
    periods
        .iter()
        .fold(0, |t, p| if t == 0 { *p } else { lcm(t, *p) })
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
