use std::cmp::Ordering;
use std::collections::BTreeSet;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn diff(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn mirrored_y(&self) -> Self {
        Self::new(self.x, -self.y)
    }

    fn normalized(&self) -> Self {
        let d = gcd(self.x, self.y).abs();
        Self::new(self.x / d, self.y / d)
    }

    fn normalized_diff(&self, other: &Self) -> Self {
        self.diff(other).normalized()
    }

    fn sector(&self) -> usize {
        if (self.x == 0 && self.y == 0) || (self.x >= 0 && self.y > 0) {
            1
        } else if self.x > 0 && self.y <= 0 {
            2
        } else if self.x <= 0 && self.y < 0 {
            3
        } else if self.x < 0 && self.y >= 0 {
            4
        } else {
            unreachable!();
        }
    }

    fn rotated_counter_clockwise(&self) -> Self {
        Self::new(-self.y, self.x)
    }

    fn rotated_clockwise(&self) -> Self {
        Self::new(self.y, -self.x)
    }

    fn rotated_to_sector_1(&self) -> Self {
        let sector = self.sector();
        match sector {
            1 => self.clone(),
            2 => self.rotated_counter_clockwise(),
            3 => self.rotated_clockwise().rotated_clockwise(),
            4 => self.rotated_clockwise(),
            _ => unreachable!(),
        }
    }

    fn abs_sq(&self) -> u64 {
        let result = self.x * self.x + self.y * self.y;
        assert!(result >= 0);
        result as u64
    }
}

fn compare_slopes(delta_a: &Point, delta_b: &Point) -> Ordering {
    if delta_a.x == 0 && delta_b.x == 0 {
        Ordering::Equal
    } else if delta_a.x == 0 {
        Ordering::Less
    } else if delta_b.x == 0{
        Ordering::Greater
    } else {
        let left = delta_a.y * delta_b.x;
        let right = delta_b.y * delta_a.x;
        right.cmp(&left)
    }
}

fn compare_angles(a: &Point, b: &Point, anchor: &Point) -> Ordering {
    let mut delta_a = a.diff(anchor).mirrored_y();
    let mut delta_b = b.diff(anchor).mirrored_y();
    let sector_a = delta_a.sector();
    let sector_b = delta_b.sector();
    match sector_a.cmp(&sector_b) {
        Ordering::Equal => {
            delta_a = delta_a.rotated_to_sector_1();
            delta_b = delta_b.rotated_to_sector_1();
            let cmp = compare_slopes(&delta_a, &delta_b);
            if cmp != Ordering::Equal {
                cmp
            } else {
                let abs_a = delta_a.abs_sq();
                let abs_b = delta_b.abs_sq();
                abs_a.cmp(&abs_b)
            }
        },
        ord => ord,
    }
}

fn parse_input(input: &str) -> BTreeSet<Point> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Point::new(x as i64, y as i64))
        })
        .collect()
}

fn find_best_location(asteroids: &BTreeSet<Point>) -> (Point, usize) {
    let mut arg_max = None;
    let mut max = None;

    for p0 in asteroids {
        let mut directions = BTreeSet::new();
        for p1 in asteroids {
            if p1 == p0 {
                continue;
            }
            let delta = p1.normalized_diff(p0);
            directions.insert(delta);
        }

        if max.is_none() || directions.len() > max.unwrap()  {
            arg_max = Some(p0.clone());
            max = Some(directions.len());
        }
    }

    (arg_max.unwrap(), max.unwrap())
}

fn compute_solution_part_one(input: &str) -> usize {
    let asteroids = parse_input(input);
    let (_, m) = find_best_location(&asteroids);
    m
}

fn compute_solution_part_two(input: &str) -> i64 {
    let asteroids = parse_input(input);
    let (anchor, _) = find_best_location(&asteroids);
    let mut others: Vec<_> = asteroids.iter().filter(|a| **a != anchor).collect();
    others.sort_by(|a, b| compare_angles(a, b, &anchor));

    let mut alive = vec![true; others.len()];
    let mut destroyed = 0;
    let mut index = 0;
    let mut last_destroyed = None;
    let mut last_direction = Point::new(0, 0);
    while destroyed < 200 {
        let direction = others[index].normalized_diff(&anchor);
        if alive[index] && last_direction != direction {
            destroyed += 1;
            alive[index] = false;
            last_destroyed = Some(&others[index]);
            last_direction = direction.clone();
        }
        index += 1;
        if index == others.len() {
            index = 0;
            last_direction = Point::new(0, 0);
        }
    }
    last_destroyed.unwrap().x * 100 + last_destroyed.unwrap().y
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d10 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
