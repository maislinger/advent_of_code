fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d13 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = \n{}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let (mut paper, folds) = parse_input(input);
    paper = paper.folded(folds[0]);
    paper.count_dots()
}

fn compute_solution_part_two(input: &str) -> String {
    let (mut paper, folds) = parse_input(input);
    for f in folds.iter() {
        paper = paper.folded(*f);
    }
    paper.to_string()
}

fn parse_input(input: &str) -> (DottedPaper, Vec<Fold>) {
    let mut max_x: usize = 0;
    let mut max_y: usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        let mut line_iter = line.split(',');
        let x = line_iter.next().unwrap().parse().unwrap();
        let y = line_iter.next().unwrap().parse().unwrap();
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
    }

    let mut paper = DottedPaper::zeros(max_x + 1, max_y + 1);

    let mut lines = input.lines();

    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let mut line_iter = line.split(',');
        let x = line_iter.next().unwrap().parse().unwrap();
        let y = line_iter.next().unwrap().parse().unwrap();
        paper.set(x, y, true);
    }

    let mut folds = Vec::new();

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();

        let mut line_iter = line.split('=');
        let text = line_iter.next().unwrap();
        let amount = line_iter.next().unwrap().parse().unwrap();

        match text {
            "fold along x" => folds.push(Fold::Vertical(amount)),
            "fold along y" => folds.push(Fold::Horizontal(amount)),
            _ => panic!("invalid input"),
        }
    }

    (paper, folds)
}

struct DottedPaper {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl DottedPaper {
    fn zeros(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![false; width * height],
        }
    }

    fn count_dots(&self) -> usize {
        self.data.iter().filter(|d| **d).count()
    }

    fn to_string(&self) -> String {
        self.data
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let x = i % self.width;
                let do_break = x == self.width - 1;
                if *c && do_break {
                    "#\n"
                } else if *c {
                    "#"
                } else if do_break {
                    ".\n"
                } else {
                    "."
                }
            })
            .collect()
    }

    fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, val: bool) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.data[y * self.width + x] = val;
    }

    fn new_size(fold: usize, old_size: usize) -> usize {
        if fold > old_size - fold - 1 {
            fold
        } else {
            old_size - fold - 1
        }
    }

    fn new_coord(c: usize, c_fold: usize, new_size: usize) -> usize {
        assert_ne!(c, c_fold);
        if c < c_fold {
            new_size - (c_fold - c)
        } else {
            new_size - (c - c_fold)
        }
    }

    fn folded(&self, fold: Fold) -> Self {
        match fold {
            Fold::Horizontal(y_fold) => self.folded_horizontal(y_fold),
            Fold::Vertical(x_fold) => self.folded_vertical(x_fold),
        }
    }

    fn folded_horizontal(&self, y_fold: usize) -> Self {
        let new_width = self.width;
        let new_height = Self::new_size(y_fold, self.height);
        let mut result = Self::zeros(new_width, new_height);

        for y in 0..self.height {
            if y == y_fold {
                continue;
            }
            let y1 = Self::new_coord(y, y_fold, new_height);
            for x in 0..self.width {
                if self.get(x, y) {
                    result.set(x, y1, true);
                }
            }
        }

        result
    }

    fn folded_vertical(&self, x_fold: usize) -> Self {
        let new_width = Self::new_size(x_fold, self.width);
        let new_height = self.height;
        let mut result = Self::zeros(new_width, new_height);

        for y in 0..self.height {
            for x in 0..self.width {
                if x == x_fold {
                    continue;
                }
                let x1 = Self::new_coord(x, x_fold, new_width);
                if self.get(x, y) {
                    result.set(x1, y, true);
                }
            }
        }

        result
    }
}

#[derive(Copy, Clone)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
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
