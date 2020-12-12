fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
    Down,
    DownRight,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum RangeType {
    ShortRange,
    LongRange,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

struct State {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl State {
    fn new(width: usize, height: usize, tiles: Vec<Tile>) -> Self {
        Self {
            width,
            height,
            tiles,
        }
    }

    fn from_str(input: &str) -> Self {
        let mut tiles = Vec::new();

        let mut width = 0;
        let mut height = 0;

        for line in input.lines() {
            height += 1;
            width = 0;
            for c in line.chars() {
                width += 1;
                let newtile = match c {
                    '.' => Tile::Floor,
                    'L' => Tile::Empty,
                    '#' => Tile::Occupied,
                    _ => panic!("unknown map tile"),
                };
                tiles.push(newtile);
            }
        }
        assert_eq!(width * height, tiles.len());
        Self::new(width, height, tiles)
    }

    fn index_to_ij(&self, index: usize) -> (usize, usize) {
        let i = index / self.width;
        let j = index % self.width;
        (i, j)
    }

    fn get_tile(&self, i: usize, j: usize) -> Tile {
        let index = i * self.width + j;
        self.tiles[index]
    }

    fn walk_from(&self, i: usize, j: usize, dir: Direction) -> Option<(usize, usize)> {
        let (delta_i, delta_j): (i64, i64) = match dir {
            Direction::Right => (0, 1),
            Direction::UpRight => (-1, 1),
            Direction::Up => (-1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::Left => (0, -1),
            Direction::DownLeft => (1, -1),
            Direction::Down => (1, 0),
            Direction::DownRight => (1, 1),
        };
        let width = self.width as i64;
        let height = self.height as i64;
        let i0 = (i as i64) + delta_i;
        let j0 = (j as i64) + delta_j;
        if i0 < 0 || i0 > height - 1 || j0 < 0 || j0 > width - 1 {
            None
        } else {
            Some((i0 as usize, j0 as usize))
        }
    }

    fn is_occupied_in_direction(
        &self,
        i: usize,
        j: usize,
        dir: Direction,
        range_type: RangeType,
    ) -> bool {
        let (mut i, mut j) = (i, j);
        loop {
            let walk_result = self.walk_from(i, j, dir);
            if walk_result.is_none() {
                return false;
            }
            i = walk_result.unwrap().0;
            j = walk_result.unwrap().1;

            match self.get_tile(i, j) {
                Tile::Occupied => return true,
                Tile::Empty => return false,
                Tile::Floor => (),
            }
            if range_type == RangeType::ShortRange {
                break;
            }
        }
        false
    }

    fn update(&mut self, range_type: RangeType) -> bool {
        let mut changed = false;
        let mut new_tiles = vec![Tile::Floor; self.tiles.len()];

        for index in 0..self.tiles.len() {
            let mut occupied_count = 0;
            let (i, j) = self.index_to_ij(index);

            for &dir in [
                Direction::Right,
                Direction::UpRight,
                Direction::Up,
                Direction::UpLeft,
                Direction::Left,
                Direction::DownLeft,
                Direction::Down,
                Direction::DownRight,
            ]
            .iter()
            {
                if self.is_occupied_in_direction(i, j, dir, range_type) {
                    occupied_count += 1;
                }
            }

            let threshold = match range_type {
                RangeType::ShortRange => 4,
                RangeType::LongRange => 5,
            };

            let current_tile = self.get_tile(i, j);
            new_tiles[index] = if current_tile == Tile::Empty && occupied_count == 0 {
                changed = true;
                Tile::Occupied
            } else if current_tile == Tile::Occupied && occupied_count >= threshold {
                changed = true;
                Tile::Empty
            } else {
                current_tile
            };
        }
        self.tiles = new_tiles;
        changed
    }

    fn update_until_stable(&mut self, range_type: RangeType) {
        let mut changed = true;
        while changed {
            changed = self.update(range_type);
        }
    }

    fn count_occupied(&self) -> usize {
        self.tiles.iter().filter(|t| **t == Tile::Occupied).count()
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut state = State::from_str(input);
    state.update_until_stable(RangeType::ShortRange);
    state.count_occupied()
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut state = State::from_str(input);
    state.update_until_stable(RangeType::LongRange);
    state.count_occupied()
}

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
