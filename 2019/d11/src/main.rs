use int_code_machine::IntCodeMachine;
use std::collections::BTreeMap;

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
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&mut self, turn_code: i64) {
        match (*self, turn_code) {
            (Direction::Up, 0) => *self = Direction::Left,
            (Direction::Up, 1) => *self = Direction::Right,
            (Direction::Down, 0) => *self = Direction::Right,
            (Direction::Down, 1) => *self = Direction::Left,
            (Direction::Left, 0) => *self = Direction::Down,
            (Direction::Left, 1) => *self = Direction::Up,
            (Direction::Right, 0) => *self = Direction::Up,
            (Direction::Right, 1) => *self = Direction::Down,
            _ => unreachable!(),
        }
    }
}

fn paint_hull(input: &str, initial_color: i64) -> BTreeMap<(i64, i64), i64> {
    let mut brain = IntCodeMachine::from_string(input);
    let mut hull = BTreeMap::new();
    let mut x = 0;
    let mut y = 0;
    hull.insert((x, y), initial_color);
    let mut direction = Direction::Up;

    loop {
        let color = hull.entry((x, y)).or_insert(0);
        brain.add_input_signal(*color);
        brain.run_until_output_or_halt();
        if brain.halted {
            break;
        }
        let new_color = brain.last_output_signal().unwrap();
        hull.insert((x, y), new_color);
        brain.run_until_output_or_halt();
        let turn_code = brain.last_output_signal().unwrap();
        direction.turn(turn_code);
        match direction {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
    }
    hull
}

fn compute_solution_part_one(input: &str) -> usize {
    let hull = paint_hull(input, 0);
    hull.len()
}

fn print_solution_part_two(input: &str) {
    let mut hull = paint_hull(input, 1);
    let top = *hull.keys().map(|(_, y)| y).max().unwrap();
    let bottom = *hull.keys().map(|(_, y)| y).min().unwrap();
    let left = *hull.keys().map(|(x, _)| x).min().unwrap();
    let right = *hull.keys().map(|(x, _)| x).max().unwrap();
    for y in (bottom..=top).rev() {
        for x in left..=right {
            let color = hull.entry((x, y)).or_insert(0);
            match color {
                0 => print!(" "),
                1 => print!("■"),
                _ => panic!("unknown color"),
            }
        }
        println!();
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
        println!("solution 2");
        print_solution_part_two(&input);
    }
}
