enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        };
    }

    fn turn_right(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        };
    }

    fn reverse(&mut self) {
        *self = match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
    }

    fn change_with_field(&mut self, field: &Field) {
        match *field {
            Field::Clean => self.turn_left(),
            Field::Infected => self.turn_right(),
            Field::Flagged => self.reverse(),
            Field::Weakened => (),
        }
    }
}

#[derive(Clone, Copy)]
enum Mode {
    Basic,
    Advanced,
}

#[derive(Clone, Copy)]
enum Field {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

impl Field {
    fn step(&mut self, mode: &Mode) {
        let newval = match (*self, *mode) {
            (Field::Clean, Mode::Basic) |
            (Field::Weakened, Mode::Advanced) => Field::Infected,
            (Field::Infected, Mode::Basic) |
            (Field::Flagged, Mode::Advanced) => Field::Clean,
            (Field::Clean, Mode::Advanced) => Field::Weakened,
            (Field::Infected, Mode::Advanced) => Field::Flagged,
            _ => unreachable!(),
        };
        *self = newval;
    }
}

#[derive(Clone)]
struct Coordinate {
    i: usize,
    height: usize,
    j: usize,
    width: usize,
}

impl Coordinate {
    fn new(i: usize, height: usize, j: usize, width: usize) -> Coordinate {
        Coordinate {
            i: i,
            height: height,
            j: j,
            width: width,
        }
    }

    fn as_ind(&self) -> usize {
        self.i * self.width + self.j
    }

    fn at_horizontal_border(&self) -> bool {
        self.j == 0 || self.j == self.width - 1
    }

    fn at_vertical_border(&self) -> bool {
        self.i == 0 || self.i == self.height - 1
    }

    fn move_to_dir(&mut self, dir: &Direction) {
        match *dir {
            Direction::Up => self.i -= 1,
            Direction::Down => self.i += 1,
            Direction::Left => self.j -= 1,
            Direction::Right => self.j += 1,
        }
    }
}

struct VirusCarrier {
    pos: Coordinate,
    dir: Direction,
    fields: Vec<Field>,
    caused_infections: usize,
}

impl VirusCarrier {
    fn new_from_fields(height: usize, width: usize, fields: Vec<Field>) -> VirusCarrier {
        let i = height / 2;
        let j = width / 2;
        let pos = Coordinate::new(i, height, j, width);
        VirusCarrier {
            pos: pos,
            dir: Direction::Up,
            fields: fields,
            caused_infections: 0,
        }
    }

    fn step(&mut self, mode: &Mode) {
        let ind = self.pos.as_ind();
        self.dir.change_with_field(&self.fields[ind]);
        self.fields[ind].step(mode);
        if let Field::Infected = self.fields[ind] {
            self.caused_infections += 1
        }
        self.pos.move_to_dir(&self.dir);
        self.check_increase_horizontal();
        self.check_increase_vertical();
    }

    fn check_increase_horizontal(&mut self) {
        if !self.pos.at_horizontal_border() {
            return;
        }

        let new_width = self.pos.width + 2;
        let new_height = self.pos.height;
        let mut new_fields = vec![Field::Clean; new_height * new_width];

        let mut small_pos = Coordinate::new(0, self.pos.height, 0, self.pos.width);
        let mut large_pos = Coordinate::new(0, new_height, 0, new_width);

        for i in 0..self.pos.height {
            small_pos.i = i;
            large_pos.i = i;
            for j in 0..self.pos.width {
                small_pos.j = j;
                large_pos.j = j + 1;
                let ind = small_pos.as_ind();
                let ind2 = large_pos.as_ind();
                new_fields[ind2] = self.fields[ind];
            }
        }

        large_pos.i = self.pos.i;
        large_pos.j = self.pos.j + 1;

        self.pos = large_pos;
        self.fields = new_fields;
    }

    fn check_increase_vertical(&mut self) {
        if !self.pos.at_vertical_border() {
            return;
        }

        let new_width = self.pos.width;
        let new_height = self.pos.height + 2;
        let mut new_fields = vec![Field::Clean; new_height * new_width];

        let mut small_pos = Coordinate::new(0, self.pos.height, 0, self.pos.width);
        let mut large_pos = Coordinate::new(0, new_height, 0, new_width);

        for i in 0..self.pos.height {
            small_pos.i = i;
            large_pos.i = i + 1;
            for j in 0..self.pos.width {
                small_pos.j = j;
                large_pos.j = j;
                let ind = small_pos.as_ind();
                let ind2 = large_pos.as_ind();
                new_fields[ind2] = self.fields[ind];
            }
        }

        large_pos.i = self.pos.i + 1;
        large_pos.j = self.pos.j;

        self.pos = large_pos;
        self.fields = new_fields;
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

fn convert_input(input: &str) -> VirusCarrier {
    let mut fields = Vec::new();
    let mut i_max = 0;
    let mut j_max = 0;
    for (i, line) in input.split('\n').enumerate() {
        let line = line.trim();
        if i > i_max {
            i_max = i;
        }
        for (j, c) in line.chars().enumerate() {
            if j > j_max {
                j_max = j;
            }
            let newfield = match c {
                '.' => Field::Clean,
                '#' => Field::Infected,
                _ => unreachable!(),
            };
            fields.push(newfield);
        }
    }
    let width = j_max + 1;
    let height = i_max + 1;
    assert_eq!(fields.len(), height * width);
    VirusCarrier::new_from_fields(height, width, fields)
}

fn compute_solution_part_one(input: &str) -> usize {
    let mode = Mode::Basic;
    let mut virus_carrier = convert_input(input);

    for _ in 0..10_000 {
        virus_carrier.step(&mode);
    }

    virus_carrier.caused_infections
}

fn compute_solution_part_two(input: &str) -> usize {
    let mode = Mode::Advanced;
    let mut virus_carrier = convert_input(input);

    for _ in 0..10_000_000 {
        virus_carrier.step(&mode);
    }

    virus_carrier.caused_infections
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "..#
                     #..
                     ...";
        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 5_587);

        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 2_511_944);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d22 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
