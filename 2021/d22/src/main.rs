fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d22 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> i64 {
    let commands = parse_input(input);
    let mut group = CuboidGroup::new();

    for c in &commands {
        if c.cuboid.max_mag() > 50 {
            continue;
        }

        match c.command_type {
            CommandType::On => group.add(&c.cuboid),
            CommandType::Off => group.remove(&c.cuboid),
        }
    }

    group.volume()
}

fn compute_solution_part_two(input: &str) -> i64 {
    let commands = parse_input(input);
    let mut group = CuboidGroup::new();

    for c in &commands {
        match c.command_type {
            CommandType::On => group.add(&c.cuboid),
            CommandType::Off => group.remove(&c.cuboid),
        }
    }

    group.volume()
}

fn parse_input(input: &str) -> Vec<Command> {
    let mut result = Vec::new();
    let input = input.trim();
    for line in input.lines() {
        result.push(Command::from_str(line));
    }

    result
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Cuboid {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
    z0: i64,
    z1: i64,
}

impl Cuboid {
    fn new(x0: i64, x1: i64, y0: i64, y1: i64, z0: i64, z1: i64) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            z0,
            z1,
        }
    }

    fn from_str(s: &str) -> Self {
        fn parse_range(s: &str) -> (i64, i64) {
            let s = s.trim();
            let s = s.split('=').nth(1).unwrap().trim();
            let mut val_iter = s.split("..");
            let v0 = val_iter.next().unwrap().parse().unwrap();
            let v1 = val_iter.next().unwrap().parse().unwrap();
            (v0, v1)
        }

        let s = s.trim();
        let mut vals = [0; 6];
        let mut index = 0;
        for sub in s.split(',') {
            let subvals = parse_range(sub);
            vals[index] = subvals.0;
            vals[index + 1] = subvals.1;
            index += 2;
        }

        Self {
            x0: vals[0],
            x1: vals[1],
            y0: vals[2],
            y1: vals[3],
            z0: vals[4],
            z1: vals[5],
        }
    }

    fn volume(&self) -> i64 {
        let dx = self.x1 - self.x0 + 1;
        let dy = self.y1 - self.y0 + 1;
        let dz = self.z1 - self.z0 + 1;
        dx * dy * dz
    }

    fn max_mag(&self) -> i64 {
        let mut r = 0;
        if self.x0.abs() > r {
            r = self.x0.abs();
        }
        if self.x1.abs() > r {
            r = self.x1.abs();
        }
        if self.y0.abs() > r {
            r = self.y0.abs();
        }
        if self.y1.abs() > r {
            r = self.y1.abs();
        }
        if self.z0.abs() > r {
            r = self.z0.abs();
        }
        if self.z1.abs() > r {
            r = self.z1.abs();
        }

        r
    }

    fn has_overlap_with(&self, other: &Self) -> bool {
        if self.x1 < other.x0 {
            return false;
        }
        if self.x0 > other.x1 {
            return false;
        }
        if self.y1 < other.y0 {
            return false;
        }
        if self.y0 > other.y1 {
            return false;
        }
        if self.z1 < other.z0 {
            return false;
        }
        if self.z0 > other.z1 {
            return false;
        }

        true
    }

    fn split_vals(
        v0: i64,
        v1: i64,
        t0: i64,
        t1: i64,
    ) -> ([Option<[i64; 2]>; 2], [i64; 2], [Option<[i64; 2]>; 2]) {
        assert!(v1 >= t0);
        assert!(v0 <= t1);

        let r0 = if v0 > t0 { v0 } else { t0 };

        let r1 = if v1 < t1 { v1 } else { t1 };

        let belongs_to_both = [r0, r1];
        let mut belongs_to_v = [None, None];
        let mut belongs_to_t = [None, None];
        let mut index_v = 0;
        let mut index_t = 0;

        if v0 < t0 {
            belongs_to_v[index_v] = Some([v0, r0 - 1]);
            index_v += 1;
        } else if t0 < v0 {
            belongs_to_t[index_t] = Some([t0, r0 - 1]);
            index_t += 1;
        }

        if v1 > t1 {
            belongs_to_v[index_v] = Some([r1 + 1, v1]);
        } else if t1 > v1 {
            belongs_to_t[index_t] = Some([r1 + 1, t1]);
        }

        (belongs_to_v, belongs_to_both, belongs_to_t)
    }

    fn split_by(&self, other: &Self) -> Vec<Self> {
        if !self.has_overlap_with(other) {
            return vec![*self];
        }

        let (to_self_x, to_both_x, _) = Self::split_vals(self.x0, self.x1, other.x0, other.x1);
        let (to_self_y, to_both_y, _) = Self::split_vals(self.y0, self.y1, other.y0, other.y1);
        let (to_self_z, _, _) = Self::split_vals(self.z0, self.z1, other.z0, other.z1);

        let mut result = Vec::new();

        for &sx in &to_self_x {
            if sx.is_none() {
                continue;
            }
            let sx = sx.unwrap();
            result.push(Self::new(sx[0], sx[1], self.y0, self.y1, self.z0, self.z1));
        }

        for &sy in &to_self_y {
            if sy.is_none() {
                continue;
            }
            let sy = sy.unwrap();
            result.push(Self::new(to_both_x[0], to_both_x[1], sy[0], sy[1], self.z0, self.z1));
        }

        for &sz in &to_self_z {
            if sz.is_none() {
                continue;
            }
            let sz = sz.unwrap();
            result.push(Self::new(to_both_x[0], to_both_x[1], to_both_y[0], to_both_y[1], sz[0], sz[1]));
        }

        result
    }
}

#[derive(Debug, Copy, Clone)]
enum CommandType {
    On,
    Off,
}

#[derive(Debug, Copy, Clone)]
struct Command {
    cuboid: Cuboid,
    command_type: CommandType,
}

impl Command {
    fn from_str(s: &str) -> Self {
        let s = s.trim();
        let mut s_iter = s.split_whitespace();
        let command_type_str = s_iter.next().unwrap();
        let cuboid = Cuboid::from_str(s_iter.next().unwrap());

        let command_type = match command_type_str {
            "on" => CommandType::On,
            "off" => CommandType::Off,
            _ => panic!("invalid input"),
        };

        Self {
            cuboid,
            command_type,
        }
    }
}

struct CuboidGroup {
    cuboids: Vec<Cuboid>,
}

impl CuboidGroup {
    fn new() -> Self {
        Self {
            cuboids: Vec::new(),
        }
    }

    fn add(&mut self, c: &Cuboid) {
        let mut sub_c = vec![*c];

        for v in &self.cuboids {
            let mut new_sub_c = Vec::new();

            for t in &sub_c {
                let mut belongs_to_t = t.split_by(v);
                new_sub_c.append(&mut belongs_to_t);
            }

            sub_c = new_sub_c;
        }

        self.cuboids.append(&mut sub_c);
    }

    fn remove(&mut self, c: &Cuboid) {
        let mut new_cuboids = Vec::new();

        for v in &self.cuboids {
            let mut belongs_to_v = v.split_by(c);
            new_cuboids.append(&mut belongs_to_v);
        }

        self.cuboids = new_cuboids;
    }

    fn volume(&self) -> i64 {
        self.cuboids.iter().map(|c| c.volume()).sum()
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
