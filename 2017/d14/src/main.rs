use std::collections::{BTreeMap, BTreeSet};

struct XorIter<'a> {
    vals: &'a [usize],
    i: usize,
}

impl<'a> XorIter<'a> {
    fn new(vals: &'a [usize]) -> XorIter<'a> {
        XorIter { vals: vals, i: 0 }
    }
}

impl<'a> Iterator for XorIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.i < 16 {
            let xor = self.vals.iter().skip(self.i * 16).take(16).fold(
                0usize,
                |acc, &x| acc ^ x,
            );
            self.i += 1;
            Some(xor)
        } else {
            None
        }
    }
}

struct BitIter<'a> {
    xor_iter: XorIter<'a>,
    xor: Option<usize>,
    from_bit: usize,
    bit: usize,
}

impl<'a> BitIter<'a> {
    fn new(vals: &'a [usize], from_bit: usize) -> BitIter {
        let mut xor_iter = XorIter::new(vals);
        let xor = xor_iter.next();
        BitIter {
            xor_iter: xor_iter,
            xor: xor,
            from_bit: from_bit,
            bit: 0,
        }
    }
}

impl<'a> Iterator for BitIter<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.bit > self.from_bit {
            self.xor = self.xor_iter.next();
            self.bit = 0;
        }

        if let Some(xor) = self.xor {
            let result = Some(xor & (1 << (self.from_bit - self.bit)) != 0);
            self.bit += 1;
            result
        } else {
            None
        }
    }
}

struct Hasher {
    position: usize,
    skip: usize,
    vals: Vec<usize>,
}

impl Hasher {
    fn new(length: usize) -> Hasher {
        let vals = (0..length).collect();
        Hasher {
            position: 0,
            skip: 0,
            vals: vals,
        }
    }

    fn step(&mut self, length: usize) {
        let n = self.vals.len();
        let mut i = self.position;
        let mut j = (self.position + length - 1) % n;

        for _ in 0..length / 2 {
            self.vals.swap(i, j);
            i = if i == n - 1 { 0 } else { i + 1 };
            j = if j == 0 { n - 1 } else { j - 1 };
        }

        self.position += length + self.skip;
        self.skip += 1;
        self.position %= n;
    }

    fn parse_str_as_numbers(&mut self, input: &str) {
        input
            .split(',')
            .map(|sr| {
                let length = sr.trim().parse().unwrap();
                self.step(length);
            })
            .count();
    }

    fn parse_str_as_ascii(&mut self, input: &str) {
        input
            .chars()
            .map(|c| {
                let c = c as u8;
                let length = c as usize;
                self.step(length);
            })
            .count();
    }

    fn xor_iter(&self) -> XorIter {
        XorIter::new(&self.vals)
    }

    fn bit_iter(&self) -> BitIter {
        BitIter::new(&self.vals, 7)
    }

    fn count_ones(&self) -> usize {
        self.xor_iter().map(|xor| xor.count_ones() as usize).sum()
    }

    fn reset(&mut self) {
        self.position = 0;
        self.skip = 0;
        self.vals = (0..self.vals.len()).collect();
    }

    fn hash(&mut self, input: &str) {
        self.reset();
        for _ in 0..64 {
            self.parse_str_as_ascii(input);
            self.parse_str_as_numbers("17,31,73,47,23");
        }
    }
}

struct GroupCounter {
    map: BTreeMap<(usize, usize), usize>,
    next_gn: usize, // next_group_number
    unused_gn: BTreeSet<usize>,
}

impl GroupCounter {
    fn new() -> GroupCounter {
        GroupCounter {
            map: BTreeMap::new(),
            next_gn: 0,
            unused_gn: BTreeSet::new(),
        }
    }

    fn start_new_group_at(&mut self, row: usize, column: usize) {
        if self.unused_gn.is_empty() {
            self.map.insert((row, column), self.next_gn);
            self.next_gn += 1;
        } else {
            let gn = *self.unused_gn.iter().nth(0).unwrap();
            self.map.insert((row, column), gn);
            self.unused_gn.remove(&gn);
        }
    }

    fn insert_topleft(&mut self) {
        self.start_new_group_at(0, 0);
    }

    fn insert_top(&mut self, column: usize) {
        let gn = match self.map.get(&(0, column - 1)) {
            Some(gn) => Some(*gn),
            None => None,
        };

        match gn {
            Some(gn) => {
                self.map.insert((0, column), gn);
            }
            None => self.start_new_group_at(0, column),
        }
    }

    fn insert_left(&mut self, row: usize) {
        let gn = match self.map.get(&(row - 1, 0)) {
            Some(gn) => Some(*gn),
            None => None,
        };

        match gn {
            Some(gn) => {
                self.map.insert((row, 0), gn);
            }
            None => self.start_new_group_at(row, 0),
        }
    }

    fn update_gn_at(&mut self, row: usize, column: usize, new_gn: usize) {
        let mut to_check = BTreeSet::new();
        to_check.insert((row, column));

        loop {
            if to_check.is_empty() {
                break;
            }

            let &(i, j) = to_check.iter().nth(0).unwrap();

            let done = match self.map.get(&(i, j)) {
                Some(gn) => *gn == new_gn,
                None => true,
            };

            if !done {
                self.map.insert((i, j), new_gn);

                if i != 0 {
                    to_check.insert((i - 1, j));
                }
                to_check.insert((i + 1, j));
                if j != 0 {
                    to_check.insert((i, j - 1));
                }
                to_check.insert((i, j + 1));
            }

            to_check.remove(&(i, j));
        }
    }

    fn insert_bulk(&mut self, row: usize, column: usize) {
        let (up, gn_up) = match self.map.get(&(row - 1, column)) {
            Some(gn) => (true, *gn),
            None => (false, self.next_gn),
        };
        let (left, gn_left) = match self.map.get(&(row, column - 1)) {
            Some(gn) => (true, *gn),
            None => (false, self.next_gn),
        };

        if !up && !left {
            self.start_new_group_at(row, column);
        } else if !up && left {
            self.map.insert((row, column), gn_left);
        } else if up && !left || gn_up == gn_left {
            self.map.insert((row, column), gn_up);
        } else if gn_up < gn_left {
            self.map.insert((row, column), gn_up);
            self.update_gn_at(row, column - 1, gn_up);
            self.unused_gn.insert(gn_left);
        } else {
            self.map.insert((row, column), gn_left);
            self.update_gn_at(row - 1, column, gn_left);
            self.unused_gn.insert(gn_up);
        }
    }

    fn insert(&mut self, row: usize, column: usize) {
        if row == 0 && column == 0 {
            self.insert_topleft();
        } else if row == 0 {
            self.insert_top(column);
        } else if column == 0 {
            self.insert_left(row);
        } else {
            self.insert_bulk(row, column);
        }
    }

    fn number_of_groups(&self) -> usize {
        self.next_gn - self.unused_gn.len()
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

fn compute_solution_part_one(input: &str) -> usize {
    let mut hasher = Hasher::new(256);

    (0..128)
        .map(|i| {
            let hash_input = format!("{}-{}", input, i);
            hasher.hash(&hash_input);
            hasher.count_ones()
        })
        .sum()
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut hasher = Hasher::new(256);
    let mut group_counter = GroupCounter::new();

    for i in 0..128 {
        let hash_input = format!("{}-{}", input, i);
        hasher.hash(&hash_input);
        for j in hasher.bit_iter().enumerate().filter(|&(_, b)| b).map(
            |(j, _)| j,
        )
        {
            group_counter.insert(i, j);
        }
    }
    // (0..128)
    //     .flat_map(|i| {
    //         let hash_input = format!("{}-{}", input, i);
    //         hasher.hash(&hash_input);
    //         hasher.bit_iter()
    //     })
    //     .count();
    group_counter.number_of_groups()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "flqrgnkx";
        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 8108);

        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 1242);

    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d14 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
