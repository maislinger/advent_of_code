fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d11 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut octopus_map = OctopusMap::from_str(input);
    let mut result = 0;

    for _ in 0..100 {
        result += octopus_map.step();
    }

    result
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut octopus_map = OctopusMap::from_str(input);
    let mut step_number = 0;
    loop {
        let n_flashes = octopus_map.step();
        step_number += 1;

        if n_flashes == octopus_map.width * octopus_map.height {
            return step_number;
        }
    }
}

struct OctopusMap {
    data: Vec<CellState>,
    width: usize,
    height: usize,
}

impl OctopusMap {
    fn from_str(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let mut data = Vec::new();
        let mut height = 0;

        for line in input.lines() {
            height += 1;
            let mut local_width = 0;
            let mut line_states = Vec::new();
            for c in line.chars() {
                local_width += 1;
                let energy_level = c.to_digit(10).unwrap() as usize;
                line_states.push(CellState::new(energy_level));
            }

            assert_eq!(local_width, width);
            data.extend(line_states);
        }

        Self {
            data,
            width,
            height,
        }
    }

    // Performs a step and returns the number of flashes
    fn step(&mut self) -> usize {
        for c in self.data.iter_mut() {
            c.energy_level += 1;
        }

        let mut changed = true;

        while changed {
            changed = false;

            for index in 0..self.data.len() {
                let i = index / self.width;
                let j = index % self.width;

                if self.data[index].energy_level < 10 || self.data[index].flash_in_current_step {
                    continue;
                }

                changed = true;
                self.data[index].flash_in_current_step = true;
                for (ni, nj) in neighbors(i, self.height, j, self.width) {
                    let n_index = ni * self.width + nj;
                    self.data[n_index].energy_level += 1;
                }
            }
        }

        self.data
            .iter_mut()
            .filter(|s| s.flash_in_current_step)
            .map(|s| {
                s.flash_in_current_step = false;
                s.energy_level = 0;
            })
            .count()
    }
}

struct CellState {
    energy_level: usize,
    flash_in_current_step: bool,
}

impl CellState {
    fn new(energy_level: usize) -> Self {
        Self {
            energy_level,
            flash_in_current_step: false,
        }
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
                2 => Some((-1, 1)),
                3 => Some((-1, 0)),
                4 => Some((-1, -1)),
                5 => Some((0, -1)),
                6 => Some((1, -1)),
                7 => Some((1, 0)),
                8 => Some((1, 1)),
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

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}
