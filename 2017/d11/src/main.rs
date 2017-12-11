enum HexDirection {
    North,
    NorthWest,
    SouthWest,
    South,
    SouthEast,
    NorthEast,
}

struct HexPosition {
    north_west: i64,
    north_east: i64,
}

impl HexPosition {
    fn new() -> HexPosition {
        HexPosition {
            north_west: 0,
            north_east: 0,
        }
    }

    fn clone(&self) -> HexPosition {
        HexPosition {
            north_west: self.north_west,
            north_east: self.north_east,
        }
    }

    fn neighbor(&self, dir: &HexDirection) -> HexPosition {
        let mut result = self.clone();

        result.north_west += match *dir {
            HexDirection::North | HexDirection::NorthWest => 1,
            HexDirection::NorthEast | HexDirection::SouthWest => 0,
            HexDirection::South | HexDirection::SouthEast => -1,
        };

        result.north_east += match *dir {
            HexDirection::North | HexDirection::NorthEast => 1,
            HexDirection::NorthWest | HexDirection::SouthEast => 0,
            HexDirection::South | HexDirection::SouthWest => -1,
        };

        result
    }

    fn steps_to_origin(&self) -> u64 {
        use std::cmp;

        let mut different_sign = false;
        different_sign = different_sign || (self.north_west > 0 && self.north_east < 0);
        different_sign = different_sign || (self.north_west < 0 && self.north_east > 0);

        if different_sign {
            self.north_west.abs() as u64 + self.north_west.abs() as u64
        } else {
            cmp::max(self.north_west.abs() as u64, self.north_east.abs() as u64)
        }
    }
}

struct HexWalker {
    pos: HexPosition,
}

impl HexWalker {
    fn new() -> HexWalker {
        HexWalker { pos: HexPosition::new() }
    }

    fn walk_to_dir(&mut self, dir: &HexDirection) {
        self.pos = self.pos.neighbor(dir);
    }

    fn parse_str(&mut self, input: &str) -> u64 {
        use std::cmp;

        input.split(',').fold(0, |max, dir_str| {
            let dir = match dir_str {
                "n" => HexDirection::North,
                "nw" => HexDirection::NorthWest,
                "sw" => HexDirection::SouthWest,
                "s" => HexDirection::South,
                "se" => HexDirection::SouthEast,
                "ne" => HexDirection::NorthEast,
                _ => unreachable!(),
            };
            self.walk_to_dir(&dir);
            let dist = self.pos.steps_to_origin();
            cmp::max(max, dist)
        })
    }

    fn steps_to_origin(&self) -> u64 {
        self.pos.steps_to_origin()
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

fn compute_solution_part_one(input: &str) -> u64 {
    let mut walker = HexWalker::new();
    walker.parse_str(input);
    walker.steps_to_origin()
}

fn compute_solution_part_two(input: &str) -> u64 {
    let mut walker = HexWalker::new();
    walker.parse_str(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let examples = vec!["ne,ne,ne", "ne,ne,sw,sw", "ne,ne,s,s", "se,sw,se,sw,sw"];
        let solutions = vec![3, 0, 2, 3];
        examples
            .iter()
            .zip(solutions.iter())
            .map(|(e, s)| assert_eq!(compute_solution_part_one(e), *s))
            .count();

        let examples = vec!["ne,ne,ne", "ne,ne,sw,sw", "ne,ne,s,s", "se,sw,se,sw,sw"];
        let solutions = vec![3, 2, 2, 3];
        examples
            .iter()
            .zip(solutions.iter())
            .map(|(e, s)| assert_eq!(compute_solution_part_two(e), *s))
            .count();
    }
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
