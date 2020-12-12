fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotated(&self, times: i64, turn_direction: TurnDirection) -> Direction {
        let mut result = *self;
        let mut times: usize = (times % 4) as usize;

        while times > 0 {
            times -= 1;
            result = match (result, turn_direction) {
                (Direction::North, TurnDirection::Left) => Direction::West,
                (Direction::North, TurnDirection::Right) => Direction::East,
                (Direction::West, TurnDirection::Left) => Direction::South,
                (Direction::West, TurnDirection::Right) => Direction::North,
                (Direction::South, TurnDirection::Left) => Direction::East,
                (Direction::South, TurnDirection::Right) => Direction::West,
                (Direction::East, TurnDirection::Left) => Direction::North,
                (Direction::East, TurnDirection::Right) => Direction::South,
            };
        }
        result
    }
}

#[derive(Clone, Copy)]
enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn inverse(&self) -> TurnDirection {
        match self {
            TurnDirection::Left => TurnDirection::Right,
            TurnDirection::Right => TurnDirection::Left,
        }
    }
}

struct Waypoint {
    x: i64,
    y: i64,
}

impl Waypoint {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn turn(&mut self, amount: i64, turn_direction: TurnDirection) {
        let mut amount = amount;
        let mut turn_direction = turn_direction;
        if amount < 0 {
            amount = -amount;
            turn_direction = turn_direction.inverse();
        }
        assert!(amount % 90 == 0);
        let mut times = (amount / 90) % 4;
        while times > 0 {
            times -= 1;
            match turn_direction {
                TurnDirection::Left => {
                    let tmp = self.x;
                    self.x = -self.y;
                    self.y = tmp;
                }
                TurnDirection::Right => {
                    let tmp = self.x;
                    self.x = self.y;
                    self.y = -tmp;
                }
            }
        }
    }
}

struct Ship {
    x: i64,
    y: i64,
    waypoint: Waypoint,
    direction: Direction,
}

impl Ship {
    fn new(x: i64, y: i64, waypoint_x: i64, waypoint_y: i64, direction: Direction) -> Self {
        let waypoint = Waypoint::new(waypoint_x, waypoint_y);
        Self {
            x,
            y,
            waypoint,
            direction,
        }
    }

    fn move_to_direction(&mut self, amount: i64, direction: Direction) {
        match direction {
            Direction::North => self.y += amount,
            Direction::South => self.y -= amount,
            Direction::East => self.x += amount,
            Direction::West => self.x -= amount,
        }
    }

    fn move_forward(&mut self, amount: i64) {
        self.move_to_direction(amount, self.direction);
    }

    fn turn(&mut self, amount: i64, turn_direction: TurnDirection) {
        let mut amount = amount;
        let mut turn_direction = turn_direction;
        if amount < 0 {
            amount = -amount;
            turn_direction = turn_direction.inverse();
        }
        assert!(amount % 90 == 0);
        let times = amount / 90;
        self.direction = self.direction.rotated(times, turn_direction);
    }

    fn parse_command_no_waypoint(&mut self, command: &str) {
        let action = command.chars().next().unwrap();
        let amount: i64 = command.chars().skip(1).collect::<String>().parse().unwrap();

        match action {
            'N' => self.move_to_direction(amount, Direction::North),
            'S' => self.move_to_direction(amount, Direction::South),
            'W' => self.move_to_direction(amount, Direction::West),
            'E' => self.move_to_direction(amount, Direction::East),
            'F' => self.move_forward(amount),
            'L' => self.turn(amount, TurnDirection::Left),
            'R' => self.turn(amount, TurnDirection::Right),
            _ => panic!("unknown action"),
        }
    }

    fn parse_command_with_waypoint(&mut self, command: &str) {
        let action = command.chars().next().unwrap();
        let amount: i64 = command.chars().skip(1).collect::<String>().parse().unwrap();

        match action {
            'N' => self.waypoint.y += amount,
            'S' => self.waypoint.y -= amount,
            'W' => self.waypoint.x -= amount,
            'E' => self.waypoint.x += amount,
            'F' => {
                self.x += amount * self.waypoint.x;
                self.y += amount * self.waypoint.y;
            }
            'L' => self.waypoint.turn(amount, TurnDirection::Left),
            'R' => self.waypoint.turn(amount, TurnDirection::Right),
            _ => panic!("unknown action"),
        }
    }

    fn distance_to_origin(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

impl Default for Ship {
    fn default() -> Self {
        Self::new(0, 0, 10, 1, Direction::East)
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut ship = Ship::default();
    for line in input.lines() {
        ship.parse_command_no_waypoint(line);
    }
    ship.distance_to_origin()
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut ship = Ship::default();
    for line in input.lines() {
        ship.parse_command_with_waypoint(line);
    }
    ship.distance_to_origin()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d12 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
