fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d15 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let chitons = Matrix::from_str(input);
    smallest_risk(&chitons)
}

fn compute_solution_part_two(input: &str) -> usize {
    let small_chitons = Matrix::from_str(input);
    let chitons = increase_size(&small_chitons);
    smallest_risk(&chitons)
}

// Compute the smalles risk to go from the top left, to the bottom right
fn smallest_risk(chitons: &Matrix<usize>) -> usize {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;

    let mut risk = Matrix::constant(chitons.width, chitons.height, None);
    risk.set(0, 0, Some(0));

    let mut todo = BinaryHeap::new();
    todo.push(Reverse(RiskEntry::new(0, 0, 0)));

    while !todo.is_empty() {
        let risk_entry = todo.pop().unwrap();

        let i = risk_entry.0.i;
        let j = risk_entry.0.j;
        let r = risk_entry.0.risk;

        if r > risk.get(i, j).unwrap() {
            continue;
        }

        for (ni, nj) in neighbors(i, chitons.height, j, chitons.width) {
            let new_r = r + chitons.get(ni, nj);
            let old_r = risk.get(ni, nj);

            let do_set = match old_r {
                Some(old_r) => old_r > new_r,
                None => true,
            };

            if do_set {
                todo.push(Reverse(RiskEntry::new(ni, nj, new_r)));
                risk.set(ni, nj, Some(new_r));
            }
        }
    }

    risk.get(risk.height - 1, risk.width - 1).unwrap()
}

fn increase_size(chitons: &Matrix<usize>) -> Matrix<usize> {
    let mut result = Matrix::constant(chitons.width * 5, chitons.height * 5, 0);

    for i in 0..result.height {
        for j in 0..result.width {
            let small_i = i % chitons.height;
            let small_j = j % chitons.width;

            let add_i = i / chitons.height;
            let add_j = j / chitons.width;

            let mut new_val = chitons.get(small_i, small_j);
            new_val += add_i + add_j;
            new_val = (new_val - 1) % 9 + 1;

            result.set(i, j, new_val);
        }
    }

    result
}

struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl Matrix<usize> {
    fn from_str(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let mut data = Vec::new();
        let mut height = 0;

        for line in input.lines() {
            height += 1;
            let line_data: Vec<usize> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            assert_eq!(line_data.len(), width);
            data.extend(line_data);
        }

        Self {
            width,
            height,
            data,
        }
    }
}

impl<T> Matrix<T> {
    fn constant(width: usize, height: usize, val: T) -> Self
    where
        T: Copy,
    {
        Self {
            width,
            height,
            data: vec![val; width * height],
        }
    }

    fn get(&self, i: usize, j: usize) -> T
    where
        T: Copy,
    {
        assert!(i < self.height);
        assert!(j < self.width);
        self.data[i * self.width + j]
    }

    fn set(&mut self, i: usize, j: usize, val: T) {
        assert!(i < self.height);
        assert!(j < self.width);
        self.data[i * self.width + j] = val;
    }
}

fn neighbors(row: usize, height: usize, col: usize, width: usize) -> NeighborIter {
    assert!(row < height);
    assert!(col < width);
    NeighborIter {
        dirnum: 0,
        width,
        height,
        i: row,
        j: col,
    }
}

struct NeighborIter {
    dirnum: usize,
    width: usize,
    height: usize,
    i: usize,
    j: usize,
}

impl Iterator for NeighborIter {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.dirnum += 1;

        loop {
            let (di, dj) = match self.dirnum {
                1 => Some((0, 1)),
                2 => Some((-1, 0)),
                3 => Some((0, -1)),
                4 => Some((1, 0)),
                _ => None,
            }?;

            let new_i = (self.i as isize) + di;
            let new_j = (self.j as isize) + dj;

            if new_i < 0
                || new_i >= (self.height as isize)
                || new_j < 0
                || new_j >= (self.width as isize)
            {
                self.dirnum += 1
            } else {
                return Some((new_i as usize, new_j as usize));
            }
        }
    }
}

#[derive(Eq, PartialEq)]
struct RiskEntry {
    i: usize,
    j: usize,
    risk: usize,
}

impl RiskEntry {
    fn new(i: usize, j: usize, risk: usize) -> Self {
        Self {
            i,
            j,
            risk,
        }
    }
}


impl PartialOrd for RiskEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering > {
        Some(self.cmp(other))
    }
}


impl Ord for RiskEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.risk < other.risk {
            std::cmp::Ordering::Less
        } else if self.risk > other.risk {
            std::cmp::Ordering::Greater
        } else {
            let dist_self = self.i + self.j;
            let dist_other = other.i + other.j;
            dist_other.cmp(&dist_self)
        }
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
