#[derive(Clone)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

impl Direction {
    fn cycle(&mut self) {
        match *self {
            Direction::Right => *self = Direction::Up,
            Direction::Up => *self = Direction::Left,
            Direction::Left => *self = Direction::Down,
            Direction::Down => *self = Direction::Right,
        }
    }
}

#[derive(Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Position {
        Position { x: x, y: y }
    }

    fn steps_to_origin(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }
}

#[derive(Clone)]
struct UlamNumber {
    nr: u64,
    pos: Position,
    dir: Direction,
    step: u64,
    threshold: u64,
    threshold_used: bool,
}

impl UlamNumber {
    fn new() -> UlamNumber {
        UlamNumber {
            nr: 1,
            pos: Position::new(0, 0),
            dir: Direction::Right,
            step: 0,
            threshold: 1,
            threshold_used: false,
        }
    }

    fn increase_by(&mut self, increment: u64) {
        assert!(self.step + increment <= self.threshold);
        self.nr += increment;
        self.step += increment;
        match self.dir {
            Direction::Right => self.pos.x += increment as i64,
            Direction::Up => self.pos.y += increment as i64,
            Direction::Left => self.pos.x -= increment as i64,
            Direction::Down => self.pos.y -= increment as i64,
        }
        if self.step == self.threshold {
            self.dir.cycle();
            self.step = 0;
            if self.threshold_used {
                self.threshold += 1;
                self.threshold_used = false;
            } else {
                self.threshold_used = true;
            }
        }
    }

    fn increase_to(&mut self, new_number: u64) {
        loop {
            let steps = self.threshold - self.step;
            if self.nr + steps > new_number {
                break;
            }
            self.increase_by(steps);
        }
        let steps = new_number - self.nr;
        self.increase_by(steps);
    }

    fn steps_to_origin(&self) -> u64 {
        self.pos.steps_to_origin()
    }

    fn compute_layer(&self) -> u64 {
        let x_abs = self.pos.x.abs() as u64;
        let y_abs = self.pos.y.abs() as u64;
        if x_abs > y_abs { x_abs } else { y_abs }
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
    let input_number = input.parse::<u64>().unwrap();
    let mut ulam_number = UlamNumber::new();
    ulam_number.increase_to(input_number);
    ulam_number.steps_to_origin()
}

fn compute_solution_part_two(input: &str) -> u64 {
    use std::collections::VecDeque;

    let input_number = input.parse::<u64>().unwrap();
    let mut ulam_numbers: VecDeque<UlamNumber> = VecDeque::new();
    let mut sums: VecDeque<u64> = VecDeque::new();
    ulam_numbers.push_back(UlamNumber::new());
    sums.push_back(1);

    loop {
        let back_layer = ulam_numbers.back().unwrap().compute_layer();
        loop {
            let front_layer = ulam_numbers.front().unwrap().compute_layer();
            if back_layer > front_layer + 1 {
                ulam_numbers.pop_front();
                sums.pop_front();
            } else {
                break;
            }
        }

        let mut new_number = ulam_numbers.back().unwrap().clone();
        new_number.increase_by(1);
        let x = new_number.pos.x;
        let y = new_number.pos.y;
        let s: u64 = ulam_numbers
            .iter()
            .zip(sums.iter())
            .filter(|&(n, _)| {
                (n.pos.x - x).abs() <= 1 && (n.pos.y - y).abs() <= 1
            })
            .map(|(&_, &s)| s)
            .sum();
        ulam_numbers.push_back(new_number);
        sums.push_back(s);
        if s > input_number {
            break;
        }
    }
    *sums.back().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        assert_eq!(compute_solution_part_one("1"), 0);
        assert_eq!(compute_solution_part_one("12"), 3);
        assert_eq!(compute_solution_part_one("23"), 2);
        assert_eq!(compute_solution_part_one("1024"), 31);

        assert_eq!(compute_solution_part_two("0"), 1);
        assert_eq!(compute_solution_part_two("55"), 57);
        assert_eq!(compute_solution_part_two("122"), 133);
        assert_eq!(compute_solution_part_two("335"), 351);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d03 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
