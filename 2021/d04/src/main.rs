fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<BingoTip>) {
    let drawn_numbers: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let mut bingo_tips = Vec::new();

    let mut line_iter = input.lines().skip(2);
    let mut line = line_iter.next();

    let mut builder = BingoTipBuilder::default();

    while line.is_some() {
        builder.feed_str(line.unwrap());

        if builder.buildable() {
            bingo_tips.push(builder.build());
            builder = BingoTipBuilder::default();
        }

        line = line_iter.next();
    }

    (drawn_numbers, bingo_tips)
}

struct BingoTip {
    numbers: [usize; 25],
    checked: [bool; 25],
}

impl BingoTip {
    fn add_draw(&mut self, n: usize) {
        for (c, &number) in self.checked.iter_mut().zip(self.numbers.iter()) {
            if number == n {
                *c = true;
            }
        }
    }

    fn is_bingo_row(&self, i: usize) -> bool {
        for j in 0..5 {
            if !self.checked[i * 5 + j] {
                return false;
            }
        }
        true
    }

    fn is_bingo_col(&self, j: usize) -> bool {
        for i in 0..5 {
            if !self.checked[i * 5 + j] {
                return false;
            }
        }
        true
    }

    fn is_bingo(&self) -> bool {
        for i in 0..5 {
            if self.is_bingo_row(i) {
                return true;
            }
        }

        for j in 0..5 {
            if self.is_bingo_col(j) {
                return true;
            }
        }
        false
    }

    fn score(&self, last_called: usize) -> usize {
        let mut s = 0;
        for (c, &number) in self.checked.iter().zip(self.numbers.iter()) {
            if !c {
                s += number;
            }
        }

        s * last_called
    }
}

struct BingoTipBuilder {
    numbers: [usize; 25],
    n_numbers_feeded: usize,
}

impl Default for BingoTipBuilder {
    fn default() -> Self {
        let numbers = [0; 25];
        let n_numbers_feeded = 0;
        Self {
            numbers,
            n_numbers_feeded,
        }
    }
}

impl BingoTipBuilder {
    fn feed_str(&mut self, s: &str) {
        for n in s.split_whitespace() {
            self.numbers[self.n_numbers_feeded] = n.parse().unwrap();
            self.n_numbers_feeded += 1;
        }
    }

    fn buildable(&self) -> bool {
        self.n_numbers_feeded == 25
    }

    fn build(self) -> BingoTip {
        assert_eq!(self.n_numbers_feeded, 25);
        BingoTip {
            numbers: self.numbers,
            checked: [false; 25],
        }
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let (drawn_numbers, mut bingo_tips) = parse_input(input);

    let mut winning_score = None;
    for &n in drawn_numbers.iter() {
        for bt in bingo_tips.iter_mut() {
            bt.add_draw(n);
            if bt.is_bingo() && winning_score.is_none() {
                winning_score = Some(bt.score(n));
                break;
            }
        }

        if winning_score.is_some() {
            break;
        }
    }

    winning_score.unwrap()
}

fn compute_solution_part_two(input: &str) -> usize {
    let (drawn_numbers, mut bingo_tips) = parse_input(input);
    let mut unfinished: Vec<usize> = (0..bingo_tips.len()).collect();
    let mut last_score = None;

    for &n in drawn_numbers.iter() {
        for &index in unfinished.iter() {
            bingo_tips[index].add_draw(n);
        }

        if unfinished.len() == 1 && bingo_tips[unfinished[0]].is_bingo() {
            last_score = Some(bingo_tips[unfinished[0]].score(n));
            break;
        }

        unfinished = unfinished
            .iter()
            .filter(|&&i| !bingo_tips[i].is_bingo())
            .cloned()
            .collect()
    }

    last_score.unwrap()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d04 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
