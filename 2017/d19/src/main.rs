use std::collections::BTreeMap;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Field {
    Vertical,
    Horizontal,
    Curve,
    Empty,
    Letter(char),
}

struct Maze {
    fields: BTreeMap<(usize, usize), Field>,
    i: usize,
    j: usize,
    width: usize,
    height: usize,
    dir: Direction,
    path_chars: Vec<char>,
    finished: bool,
    n_steps: usize,
}

impl Maze {
    fn new(fields: BTreeMap<(usize, usize), Field>) -> Maze {
        let mut result = Maze {
            fields: fields,
            i: 0,
            j: 0,
            width: 0,
            height: 0,
            dir: Direction::Down,
            path_chars: Vec::new(),
            finished: false,
            n_steps: 0,
        };
        result.reset();
        result
    }

    fn reset(&mut self) {
        let width = self.fields.keys().map(|&(_, j)| j).max().unwrap() + 1;
        let height = self.fields.keys().map(|&(i, _)| i).max().unwrap() + 1;
        self.width = width;
        self.height = height;
        self.dir = Direction::Down;
        self.path_chars = Vec::new();
        self.i = 0;
        self.finished = false;
        self.n_steps = 0;

        for j in 0..width {
            if let Some(field) = self.fields.get(&(0, j)) {
                if let Field::Vertical = *field {
                    self.j = j;
                    break;
                }
            }
        }
    }

    fn new_pos(&self) -> Option<(usize, usize)> {
        match self.dir {
            Direction::Up => {
                if self.i > 0 {
                    Some((self.i - 1, self.j))
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.i < self.height - 1 {
                    Some((self.i + 1, self.j))
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.j > 0 {
                    Some((self.i, self.j - 1))
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.j < self.width - 1 {
                    Some((self.i, self.j + 1))
                } else {
                    None
                }
            }
        }
    }

    fn priority_horizontal(&self, ci: usize, cj: usize) -> usize {
        let candidate = (ci, cj);
        if let Some(field) = self.fields.get(&candidate) {
            match *field {
                Field::Curve | Field::Letter(_) => 1,
                Field::Horizontal => 2,
                Field::Vertical | Field::Empty => 0,
            }
        } else {
            0
        }
    }

    fn priority_vertical(&self, ci: usize, cj: usize) -> usize {
        let candidate = (ci, cj);
        if let Some(field) = self.fields.get(&candidate) {
            match *field {
                Field::Curve | Field::Letter(_) => 1,
                Field::Vertical => 2,
                Field::Horizontal | Field::Empty => 0,
            }
        } else {
            0
        }
    }

    fn new_dir(&self, new_i: usize, new_j: usize) -> Direction {
        match self.dir {
            Direction::Up | Direction::Down => {
                let left = if new_j > 0 {
                    self.priority_horizontal(new_i, new_j - 1)
                } else {
                    0
                };
                let right = if new_j < self.width - 1 {
                    self.priority_horizontal(new_i, new_j + 1)
                } else {
                    0
                };
                assert!(left != right);
                if left > right {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Left | Direction::Right => {
                let up = if new_i > 0 {
                    self.priority_vertical(new_i - 1, new_j)
                } else {
                    0
                };
                let down = if new_i < self.height - 1 {
                    self.priority_vertical(new_i + 1, new_j)
                } else {
                    0
                };
                assert!(up != down);
                if up > down {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        }
    }

    fn move_pos(&mut self) {
        if self.finished {
            return;
        }

        let new_pos = self.new_pos();

        match new_pos {
            Some((new_i, new_j)) => {
                match self.fields.get(&(new_i, new_j)) {
                    Some(&Field::Curve) => self.dir = self.new_dir(new_i, new_j),
                    Some(&Field::Letter(c)) => self.path_chars.push(c),
                    Some(&Field::Empty) => unreachable!(),
                    None => self.finished = true,
                    _ => (),
                }
                self.i = new_i;
                self.j = new_j;
                self.n_steps += 1;
            }
            None => self.finished = true,
        }
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
    contents.to_owned()
}

fn convert_input(input: &str) -> BTreeMap<(usize, usize), Field> {
    let mut result = BTreeMap::new();

    for (i, line) in input.split('\n').enumerate() {
        for (j, c) in line.chars().enumerate() {
            let field = match c {
                '|' => Field::Vertical,
                '-' => Field::Horizontal,
                '+' => Field::Curve,
                ' ' => Field::Empty,
                _ => Field::Letter(c),
            };

            match field {
                Field::Empty => None,
                _ => result.insert((i, j), field),
            };
        }
    }

    result
}

fn compute_solution_part_one(input: &str) -> String {
    let fields = convert_input(input);
    let mut maze = Maze::new(fields);

    while !maze.finished {
        maze.move_pos();
    }

    maze.path_chars.iter().collect()
}

fn compute_solution_part_two(input: &str) -> usize {
    let fields = convert_input(input);
    let mut maze = Maze::new(fields);

    while !maze.finished {
        maze.move_pos();
    }

    maze.n_steps
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "
                          |
                          |  +--+
                          A  |  C
                      F---|----E|--+
                          |  |  |  D
                          +B-+  +--+";

        let input = input.split('\n').skip(1).collect::<Vec<&str>>().join("\n");

        let solution = compute_solution_part_one(&input);
        assert_eq!(solution, "ABCDEF");

        let solution = compute_solution_part_two(&input);
        assert_eq!(solution, 38);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d19 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
