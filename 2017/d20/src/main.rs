extern crate regex;

use std::cmp::Ordering;
use std::ops;

#[derive(Clone, Debug)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn new(x: i64, y: i64, z: i64) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    fn from_slice(vals: &[i64]) -> Vector {
        Vector::new(vals[0], vals[1], vals[2])
    }

    fn manhattan_abs(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64 + self.z.abs() as u64
    }
}

impl<'a> ops::AddAssign<&'a Vector> for Vector {
    fn add_assign(&mut self, other: &Vector) {
        *self = Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        self.x == other.x && self.y == other.y && self.y == other.y
    }
}

#[derive(Clone, Debug)]
struct Particle {
    p: Vector,
    v: Vector,
    a: Vector,
    id: usize,
}

impl Particle {
    fn from_slice(vals: &[i64], id: usize) -> Particle {
        let p = Vector::from_slice(&vals[..3]);
        let v = Vector::from_slice(&vals[3..6]);
        let a = Vector::from_slice(&vals[6..9]);
        Particle {
            p: p,
            v: v,
            a: a,
            id: id,
        }
    }

    fn step(&mut self) {
        self.v += &self.a;
        self.p += &self.v;
    }

    fn stable(&self) -> bool {
        let mut x = self.v.x.signum() == self.a.x.signum();
        x = x || self.a.x == 0;
        x = x && self.p.x.signum() == self.v.x.signum();
        x = x || (self.a.x == 0 && self.v.x == 0);
        let mut y = self.v.y.signum() == self.a.y.signum();
        y = y || self.a.y == 0;
        y = y && self.p.y.signum() == self.v.y.signum();
        y = y || (self.a.y == 0 && self.v.y == 0);
        let mut z = self.v.z.signum() == self.a.z.signum();
        z = z || self.a.z == 0;
        z = z && self.p.z.signum() == self.v.z.signum();
        z = z || (self.a.z == 0 && self.v.z == 0);
        x && y && z
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

fn convert_input(input: &str) -> Vec<Particle> {
    use regex::Regex;

    let co = "(-?[0-9]+)";
    let s = format!(
        "p=<{},{},{}>, v=<{},{},{}>, a=<{},{},{}>",
        co,
        co,
        co,
        co,
        co,
        co,
        co,
        co,
        co
    );
    let re: Regex = Regex::new(&s).unwrap();

    let mut result = Vec::new();

    for (id, cap) in re.captures_iter(input).enumerate() {
        let vals = cap.iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse().unwrap())
            .collect::<Vec<i64>>();
        assert_eq!(vals.len(), 9);
        let particle = Particle::from_slice(&vals, id);
        result.push(particle);
    }

    result
}

fn compute_solution_part_one(input: &str) -> usize {
    let particles = convert_input(input);
    let min = particles.iter().map(|p| p.a.manhattan_abs()).min().unwrap();
    let min_particles: Vec<_> = particles
        .iter()
        .filter(|&p| p.a.manhattan_abs() == min)
        .collect();
    if min_particles.len() == 1 {
        min_particles[0].id
    } else {
        unimplemented!();
    }
}

fn compute_solution_part_two(input: &str) -> usize {
    use std::collections::BTreeSet;

    let mut particles = convert_input(input);
    let mut result = 0;

    while !particles.is_empty() {
        particles.sort_by(|p, q| {
            let mut result = p.p.manhattan_abs().cmp(&q.p.manhattan_abs());
            let mut done = result != Ordering::Equal;
            if !done {
                result = p.p.x.cmp(&q.p.x);
                done = result != Ordering::Equal;
            }
            if !done {
                result = p.p.y.cmp(&q.p.y);
                done = result != Ordering::Equal;
            }
            if !done {
                result = p.p.z.cmp(&q.p.z);
            }
            result
        });

        let indices: BTreeSet<_> = particles
            .iter()
            .zip(particles.iter().skip(1))
            .enumerate()
            .filter(|&(_, (p1, p2))| p1.p == p2.p)
            .map(|(i, (_, _))| i)
            .collect();

        let tmp = particles
            .iter()
            .enumerate()
            .filter(|&(i, _)| !indices.contains(&i))
            .filter(|&(i, _)| i == 0 || !indices.contains(&(i - 1)))
            .map(|(_, p)| p.clone())
            .collect();

        particles = tmp;

        let id_p = particles
            .iter()
            .max_by_key(|p| p.p.manhattan_abs())
            .unwrap()
            .id;
        let id_v = particles
            .iter()
            .max_by_key(|p| p.v.manhattan_abs())
            .unwrap()
            .id;
        let id_a = particles
            .iter()
            .max_by_key(|p| p.a.manhattan_abs())
            .unwrap()
            .id;

        if id_p == id_v && id_v == id_a && particles.last().unwrap().stable() {
            particles.pop();
            result += 1;
        }

        particles.iter_mut().map(|p| p.step()).count();
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
                     p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";
        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 0);

        let input = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
                     p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
                     p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
                     p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";
        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 1);
    }
}

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
