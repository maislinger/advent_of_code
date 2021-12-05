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

fn parse_input(input: &str) -> Vec<Line> {
    fn parse_tuple(t: &str) -> (i64, i64) {
        let mut iter = t.split(',');
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        (x, y)
    }

    fn parse_line(line: &str) -> (i64, i64, i64, i64) {
        let mut iter = line.split(" -> ");
        let start = iter.next();
        let end = iter.next();
        assert!(start.is_some());
        assert!(end.is_some());

        let (x1, y1) = parse_tuple(start.unwrap());
        let (x2, y2) = parse_tuple(end.unwrap());
        (x1, y1, x2, y2)
    }

    let mut result = Vec::new();

    for line in input.lines() {
        let (x1, y1, x2, y2) = parse_line(line);
        result.push(Line::from_coords(x1, y1, x2, y2));
    }

    result
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_coords(x1: i64, y1: i64, x2: i64, y2: i64) -> Self {
        let start = Point::new(x1, y1);
        let end = Point::new(x2, y2);
        Self { start, end }
    }

    fn is_horizontal(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_vertical(&self) -> bool {
        self.start.y == self.end.y
    }

    fn iter(&self) -> LineIter {
        LineIter::new(*self)
    }
}

struct LineIter {
    line: Line,
    current: Point,
    yielded_first: bool,
}

impl LineIter {
    fn new(line: Line) -> Self {
        let current = line.start;
        let yielded_first = false;
        Self {
            line,
            current,
            yielded_first,
        }
    }
}

impl Iterator for LineIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.yielded_first {
            self.yielded_first = true;
            return Some(self.current);
        }

        if self.current == self.line.end {
            return None;
        }

        if self.line.end.x > self.line.start.x {
            self.current.x += 1;
        }

        if self.line.end.x < self.line.start.x {
            self.current.x -= 1;
        }

        if self.line.end.y > self.line.start.y {
            self.current.y += 1;
        }

        if self.line.end.y < self.line.start.y {
            self.current.y -= 1;
        }

        Some(self.current)
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let lines = parse_input(input);

    let mut full_lines: BTreeMap<Point, usize> = BTreeMap::new();

    for line in lines.iter() {
        if !line.is_horizontal() && !line.is_vertical() {
            continue;
        }

        for p in line.iter() {
            let counter = full_lines.entry(p).or_insert(0);
            *counter += 1;
        }
    }

    let mut result = 0;

    for (_, &c) in full_lines.iter() {
        if c > 1 {
            result += 1;
        }
    }

    result
}

fn compute_solution_part_two(input: &str) -> usize {
    let lines = parse_input(input);

    let mut full_lines: BTreeMap<Point, usize> = BTreeMap::new();

    for line in lines.iter() {
        for p in line.iter() {
            let counter = full_lines.entry(p).or_insert(0);
            *counter += 1;
        }
    }

    let mut result = 0;

    for (_, &c) in full_lines.iter() {
        if c > 1 {
            result += 1;
        }
    }

    result
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d05 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
