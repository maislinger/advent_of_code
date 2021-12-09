fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d09 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let map = CaveMap::from_str(input);
    let mut s = 0;

    for i in 0..map.height {
        for j in 0..map.height {
            if map.is_lowpoint(i, j) {
                s += map.get(i, j) as usize;
                s += 1;
            }
        }
    }

    s
}

fn compute_solution_part_two(input: &str) -> usize {
    let map = CaveMap::from_str(input);
    let mut three_largest = [0usize; 3];

    fn maybe_insert_value(newval: usize, three_largest: &mut [usize; 3]) {
        if newval > three_largest[0] {
            three_largest[2] = three_largest[1];
            three_largest[1] = three_largest[0];
            three_largest[0] = newval;
        } else if newval > three_largest[1] {
            three_largest[2] = three_largest[1];
            three_largest[1] = newval;
        } else if newval > three_largest[2] {
            three_largest[2] = newval;
        }
    }

    for i in 0..map.height {
        for j in 0..map.height {
            let basin_size = map.basin_size(i, j);
            maybe_insert_value(basin_size, &mut three_largest);
        }
    }

    three_largest[0] * three_largest[1] * three_largest[2]
}

struct CaveMap {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

impl CaveMap {
    fn from_str(input: &str) -> Self {
        let mut tiles = Vec::new();
        let width = input.lines().next().unwrap().chars().count();
        let mut height = 0;

        for line in input.lines() {
            height += 1;
            let local_tiles: Vec<u8> = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect();
            assert_eq!(local_tiles.len(), width);
            tiles.extend(local_tiles);
        }

        Self {
            tiles,
            width,
            height,
        }
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        assert!(row < self.height);
        assert!(col < self.width);
        self.tiles[row * self.width + col]
    }

    fn is_lowpoint(&self, row: usize, col: usize) -> bool {
        let val = self.get(row, col);

        for (i, j) in neighbors(row, self.height, col, self.width) {
            let neighbor_val = self.get(i, j);
            if neighbor_val <= val {
                return false;
            }
        }

        true
    }

    // Count basin size if (row, col) is a lowpoint
    // Otherwise, return 0
    fn basin_size(&self, row: usize, col: usize) -> usize {
        use std::collections::BTreeSet;

        if !self.is_lowpoint(row, col) {
            return 0;
        }

        let mut n_lowpoints = 0;

        let mut todo = vec![(row, col)];
        let mut done = BTreeSet::new();

        while !todo.is_empty() {
            let (i, j) = todo.pop().unwrap();

            if done.contains(&(i, j)) {
                continue;
            }
            done.insert((i, j));

            if self.is_lowpoint(i, j) {
                n_lowpoints += 1;
            }

            for (ni, nj) in neighbors(i, self.height, j, self.width) {
                if done.contains(&(ni, nj)) {
                    continue;
                }

                let val = self.get(ni, nj);
                if val != 9u8 {
                    todo.push((ni, nj));
                }
            }
        }

        assert_eq!(n_lowpoints, 1, "not implemented");

        done.len()
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
        if self.dirnum == 1 {
            if self.j < self.width - 1 {
                return Some((self.i, self.j + 1));
            } else {
                self.dirnum += 1;
            }
        }

        if self.dirnum == 2 {
            if self.i > 0 {
                return Some((self.i - 1, self.j));
            } else {
                self.dirnum += 1;
            }
        }

        if self.dirnum == 3 {
            if self.j > 0 {
                return Some((self.i, self.j - 1));
            } else {
                self.dirnum += 1;
            }
        }

        if self.dirnum == 4 {
            if self.i < self.height - 1 {
                return Some((self.i + 1, self.j));
            } else {
                self.dirnum += 1;
            }
        }

        None
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
