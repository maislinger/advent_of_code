fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d25 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut cucumbers = SeaCucumbers::from_str(input);
    let mut n_steps = 0;
    while cucumbers.step() {
        n_steps += 1;
    }
    n_steps + 1
}

struct SeaCucumbers {
    width: usize,
    height: usize,
    data: Vec<Option<Cucumber>>,
}

impl SeaCucumbers {
    fn from_str(input: &str) -> Self {
        let width = input.lines().next().unwrap().chars().count();
        let mut height = 0;
        let mut data = Vec::new();
        for line in input.lines() {
            height += 1;
            let mut line_vec: Vec<Option<Cucumber>> = line
                .chars()
                .map(|c| match c {
                    '>' => Some(Cucumber::East),
                    'v' => Some(Cucumber::South),
                    '.' => None,
                    _ => panic!("invalid input"),
                })
                .collect();

            assert_eq!(width, line_vec.len());
            data.append(&mut line_vec);
        }

        Self {
            width,
            height,
            data,
        }
    }

    fn get(&self, i: usize, j: usize) -> Option<Cucumber> {
        assert!(i < self.height);
        assert!(j < self.width);
        self.data[i * self.width + j]
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut Option<Cucumber> {
        assert!(i < self.height);
        assert!(j < self.width);
        &mut self.data[i * self.width + j]
    }

    fn get_east(&self, i: usize, j: usize) -> Option<Cucumber> {
        assert!(i < self.height);
        assert!(j < self.width);
        let mut nj = j + 1;
        if nj >= self.width {
            nj = 0;
        }

        self.data[i * self.width + nj]
    }

    fn get_east_mut(&mut self, i: usize, j: usize) -> &mut Option<Cucumber> {
        assert!(i < self.height);
        assert!(j < self.width);
        let mut nj = j + 1;
        if nj >= self.width {
            nj = 0;
        }

        &mut self.data[i * self.width + nj]
    }

    fn get_south(&self, i: usize, j: usize) -> Option<Cucumber> {
        assert!(i < self.height);
        assert!(j < self.width);
        let mut ni = i + 1;
        if ni >= self.height {
            ni = 0;
        }

        self.data[ni * self.width + j]
    }

    fn get_south_mut(&mut self, i: usize, j: usize) -> &mut Option<Cucumber> {
        assert!(i < self.height);
        assert!(j < self.width);
        let mut ni = i + 1;
        if ni >= self.height {
            ni = 0;
        }

        &mut self.data[ni * self.width + j]
    }

    fn step(&mut self) -> bool {
        let east = self.step_east();
        let south = self.step_south();
        east || south
    }

    fn step_east(&mut self) -> bool {
        let mut change = false;

        for i in 0..self.height {
            for j in 0..self.width {
                if self.get(i, j) != Some(Cucumber::East) {
                    continue;
                }

                if self.get_east(i, j) == None {
                    change = true;
                    *self.get_mut(i, j) = Some(Cucumber::EastToBeMoved);
                }
            }
        }

        for i in 0..self.height {
            for j in 0..self.width {
                if self.get(i, j) != Some(Cucumber::EastToBeMoved) {
                    continue;
                }

                *self.get_mut(i, j) = None;
                *self.get_east_mut(i, j) = Some(Cucumber::East);
            }
        }

        change
    }

    fn step_south(&mut self) -> bool {
        let mut change = false;

        for i in 0..self.height {
            for j in 0..self.width {
                if self.get(i, j) != Some(Cucumber::South) {
                    continue;
                }

                if self.get_south(i, j) == None {
                    change = true;
                    *self.get_mut(i, j) = Some(Cucumber::SouthToBeMoved);
                }
            }
        }

        for i in 0..self.height {
            for j in 0..self.width {
                if self.get(i, j) != Some(Cucumber::SouthToBeMoved) {
                    continue;
                }

                *self.get_mut(i, j) = None;
                *self.get_south_mut(i, j) = Some(Cucumber::South);
            }
        }

        change
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Cucumber {
    East,
    EastToBeMoved,
    South,
    SouthToBeMoved,
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
