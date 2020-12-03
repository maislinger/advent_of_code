use std::collections::BTreeSet;

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(Eq, PartialEq)]
enum Square {
    Open,
    Tree,
}

struct Field {
    height: usize,
    width: usize,
    trees: BTreeSet<(usize, usize)>,
}

impl Field {
    fn new(height: usize, width: usize, trees: BTreeSet<(usize, usize)>) -> Self {
        Self {
            height,
            width,
            trees,
        }
    }

    fn read_at_position(&self, i: usize, j: usize) -> Square {
        assert!(i < self.height);
        let read_j = j % self.width;
        if self.trees.contains(&(i, read_j)) {
            Square::Tree
        } else {
            Square::Open
        }
    }
}

fn parse_input(input: &str) -> Field {
    let mut trees = BTreeSet::new();
    let mut height = 0;
    let mut width = 0;
    for (i, line) in input.split_whitespace().enumerate() {
        height = i;
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                trees.insert((i, j));
            }
            width = j;
        }
    }
    height += 1;
    width += 1;
    Field::new(height, width, trees)
}

fn count_trees(delta_i: usize, delta_j: usize, field: &Field) -> usize {
    let mut i = 0;
    let mut j = 0;
    let mut result = 0;

    while i < field.height {
        if field.read_at_position(i, j) == Square::Tree {
            result += 1
        }
        i += delta_i;
        j += delta_j;
    }
    result
}

fn compute_solution_part_one(input: &str) -> usize {
    let field = parse_input(input);
    count_trees(1, 3, &field)
}

fn compute_solution_part_two(input: &str) -> usize {
    let field = parse_input(input);
    let mut result = 1;
    for &(delta_i, delta_j) in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)].iter() {
        result *= count_trees(delta_i, delta_j, &field);
    }
    result
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
