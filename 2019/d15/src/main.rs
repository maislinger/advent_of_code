use int_code_machine::IntCodeMachine;
use std::collections::{BTreeMap, BTreeSet};

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_machine_command(self) -> i64 {
        match &self {
            Direction::North => 1,
            Direction::East => 4,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    fn inverted(self) -> Self {
        match &self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn array_index(self) -> usize {
        match &self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn to_direction(&self, direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }

    fn relative_direction(&self, other: Point) -> Direction {
        if self.x + 1 == other.x {
            assert_eq!(self.y, other.y);
            Direction::East
        } else if self.x - 1 == other.x {
            assert_eq!(self.y, other.y);
            Direction::West
        } else if self.y + 1 == other.y {
            assert_eq!(self.x, other.x);
            Direction::North
        } else if self.y - 1 == other.y {
            assert_eq!(self.x, other.x);
            Direction::South
        } else {
            panic!("Points are not neighbors");
        }
    }
}

struct Board {
    data: BTreeMap<Point, [Option<Point>; 4]>,
}

impl Board {
    fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    fn add(&mut self, a: Point, b: Point) {
        let a_to_b = a.relative_direction(b);
        let b_to_a = a_to_b.inverted();
        let info_a = self.data.entry(a).or_insert([None; 4]);
        info_a[a_to_b.array_index()] = Some(b);
        let info_b = self.data.entry(b).or_insert([None; 4]);
        info_b[b_to_a.array_index()] = Some(a);
    }

    fn neighbors(&self, p: Point) -> NeighborIterator {
        let neighbors = self.data[&p];
        NeighborIterator::new(neighbors)
    }
}

struct NeighborIterator {
    neighbors: [Option<Point>; 4],
    index: usize,
}

impl NeighborIterator {
    fn new(neighbors: [Option<Point>; 4]) -> Self {
        Self {
            neighbors,
            index: 0,
        }
    }
}

impl Iterator for NeighborIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.neighbors.len() {
                return None;
            } else {
                self.index += 1;
                let i = self.index - 1;
                if let Some(p) = self.neighbors[i] {
                    return Some(p);
                }
            }
        }
    }
}

fn manhattan_heuristic(a: Point, b: Point) -> i64 {
    let delta_x = a.x - b.x;
    let delta_y = a.y - b.y;
    delta_x.abs() + delta_y.abs()
}

fn best_path(origin: Point, goal: Point, board: &Board) -> Vec<Direction> {
    use std::iter::successors;

    let mut candidates = BTreeMap::new();
    candidates.insert(origin, manhattan_heuristic(origin, goal));
    let mut parents = BTreeMap::new();
    let mut visited = BTreeSet::new();

    while !candidates.is_empty() {
        let candidate = *candidates.iter().min_by_key(|(_, v)| *v).unwrap().0;
        if candidate == goal {
            break;
        }
        for neighbor in board.neighbors(candidate) {
            if visited.contains(&neighbor) {
                continue;
            }
            parents.insert(neighbor, candidate);
            candidates.insert(neighbor, manhattan_heuristic(neighbor, goal));
        }
        visited.insert(candidate);
        candidates.remove(&candidate);
    }

    let mut points: Vec<_> = successors(Some(goal), |p| parents.get(p).cloned()).collect();
    points.reverse();
    points
        .windows(2)
        .map(|p| p[0].relative_direction(p[1]))
        .collect()
}

struct Droid {
    brain: IntCodeMachine,
    board: Board,
    position: Point,
    oxygen: Option<Point>,
}

impl Droid {
    fn from_string(input: &str) -> Self {
        Self {
            brain: IntCodeMachine::from_string(input),
            board: Board::new(),
            position: Point::new(0, 0),
            oxygen: None,
        }
    }

    fn move_to_direction(&mut self, direction: Direction) -> bool {
        self.brain.default_input = Some(direction.to_machine_command());
        self.brain.run_until_output_or_halt();
        self.brain.default_input = None;
        let result = self.brain.last_output_signal().unwrap();
        match result {
            1 => {
                self.position = self.position.to_direction(direction);
                true
            }
            2 => {
                self.position = self.position.to_direction(direction);
                self.oxygen = Some(self.position);
                true
            }
            0 => false,
            _ => panic!("unknown tile code"),
        }
    }

    fn move_to_point(&mut self, p: Point) {
        if self.position == p {
            return;
        }
        let directions = best_path(self.position, p, &self.board);
        for direction in directions {
            self.move_to_direction(direction);
        }
    }

    fn explore_board(&mut self) {
        let origin = Point::new(0, 0);
        let mut todo = Vec::new();
        todo.push((origin, Direction::North));
        todo.push((origin, Direction::West));
        todo.push((origin, Direction::South));
        todo.push((origin, Direction::East));

        let mut checked = BTreeSet::new();

        while !todo.is_empty() {
            let (p, dir) = todo.pop().unwrap();
            if checked.contains(&(p, dir)) {
                continue;
            }

            self.move_to_point(p);
            let connected = self.move_to_direction(dir);
            let new_p = self.position;
            for &new_dir in [
                Direction::North,
                Direction::West,
                Direction::South,
                Direction::East,
            ]
            .iter()
            {
                if connected && !checked.contains(&(new_p, new_dir)) {
                    todo.push((new_p, new_dir));
                }
            }

            checked.insert((p, dir));
            if connected {
                checked.insert((new_p, dir.inverted()));
                self.board.add(p, new_p);
            }
        }
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut droid = Droid::from_string(input);
    droid.explore_board();
    let oxygen = droid.oxygen.unwrap();
    let origin = Point::new(0, 0);
    best_path(origin, oxygen, &droid.board).len()
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut droid = Droid::from_string(input);
    droid.explore_board();
    let oxygen = droid.oxygen.unwrap();

    let mut filled = BTreeSet::new();
    let mut todo = Vec::new();
    todo.push(oxygen);

    let mut minutes = 0;

    loop {
        let mut new_todo = Vec::new();
        while !todo.is_empty() {
            let p = todo.pop().unwrap();
            filled.insert(p);
            for n in droid.board.neighbors(p) {
                if !filled.contains(&n) {
                    new_todo.push(n);
                }
            }
        }
        todo = new_todo;
        if todo.is_empty() {
            break;
        }
        minutes += 1;
    }

    minutes
}

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
