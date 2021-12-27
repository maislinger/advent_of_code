fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d18 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> u64 {
    let mut s = Pair::from_str(input.lines().next().unwrap());

    for line in input.lines().skip(1) {
        let n = Pair::from_str(line);
        s = s.add(n);
    }

    s.magnitude()
}

fn compute_solution_part_two(input: &str) -> u64 {
    let mut max = None;

    for a in input.lines() {
        for b in input.lines() {
            let na = Pair::from_str(a);
            let nb = Pair::from_str(b);
            let c = na.add(nb);

            let mag = c.magnitude();
            if max.is_none() || max.unwrap() < mag {
                max = Some(mag);
            }
        }
    }

    max.unwrap()
}

#[derive(Debug)]
enum PairOrNumber {
    N(u64),
    P(Pair),
}

impl PairOrNumber {
    fn unwrap_number(&self) -> u64 {
        match self {
            PairOrNumber::N(n) => *n,
            PairOrNumber::P(_) => panic!("tried to unwrap P"),
        }
    }

    fn unwrap_number_mut(&mut self) -> &mut u64 {
        match self {
            PairOrNumber::N(n) => n,
            PairOrNumber::P(_) => panic!("tried to unwrap P"),
        }
    }

    fn unwrap_pair_mut(&mut self) -> &mut Pair {
        match self {
            PairOrNumber::N(_) => panic!("tried to unwrap N"),
            PairOrNumber::P(p) => p,
        }
    }

    fn is_number(&self) -> bool {
        match self {
            PairOrNumber::N(_) => true,
            PairOrNumber::P(_) => false,
        }
    }

    fn is_pair(&self) -> bool {
        match self {
            PairOrNumber::N(_) => false,
            PairOrNumber::P(_) => true,
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: Box<PairOrNumber>,
    right: Box<PairOrNumber>,
}

impl Pair {
    fn from_str(line: &str) -> Self {
        let mut left = None;
        let mut right = None;
        for (s, is_subpair) in nested_list_split(line) {
            assert!(!(left.is_some() && right.is_some()));
            let side = if left.is_none() {
                &mut left
            } else {
                &mut right
            };

            if is_subpair {
                *side = Some(PairOrNumber::P(Self::from_str(s)));
            } else {
                *side = Some(PairOrNumber::N(s.parse().unwrap()));
            }
        }

        Self {
            left: Box::new(left.unwrap()),
            right: Box::new(right.unwrap()),
        }
    }

    fn magnitude(&self) -> u64 {
        let leftval = match &*self.left {
            PairOrNumber::N(n) => *n,
            PairOrNumber::P(p) => p.magnitude(),
        };

        let rightval = match &*self.right {
            PairOrNumber::N(n) => *n,
            PairOrNumber::P(p) => p.magnitude(),
        };

        3 * leftval + 2 * rightval
    }

    fn add(self, other: Self) -> Self {
        let mut result = Self {
            left: Box::new(PairOrNumber::P(self)),
            right: Box::new(PairOrNumber::P(other)),
        };

        result.reduce();
        result
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }

            if self.split() {
                continue;
            }

            break;
        }
    }

    fn explode(&mut self) -> bool {
        let level = 0;
        if let PairOrNumber::P(p) = &mut *self.left {
            let (was_exploding, destroy_child, _retleft, mut retright) = p.explode_inner(level + 1);
            if let Some(n) = retright {
                let mut p = &mut *self.right;
                while p.is_pair() {
                    p = &mut *p.unwrap_pair_mut().left;
                }
                *p.unwrap_number_mut() += n;
                retright = None;
            }

            if destroy_child {
                self.left = Box::new(PairOrNumber::N(0));
            }

            if was_exploding {
                return true;
            }
        }

        if let PairOrNumber::P(p) = &mut *self.right {
            let (was_exploding, destroy_child, mut retleft, _retright) = p.explode_inner(level + 1);
            if let Some(n) = retleft {
                let mut p = &mut *self.left;
                while p.is_pair() {
                    p = &mut *p.unwrap_pair_mut().right;
                }
                *p.unwrap_number_mut() += n;
                retleft = None;
            }

            if destroy_child {
                self.right = Box::new(PairOrNumber::N(0));
            }

            if was_exploding {
                return true;
            }
        }

        false
    }

    fn explode_inner(&mut self, level: usize) -> (bool, bool, Option<u64>, Option<u64>) {
        if level > 3 && self.left.is_number() && self.right.is_number() {
            return (
                true,
                true,
                Some(self.left.unwrap_number()),
                Some(self.right.unwrap_number()),
            );
        }

        if let PairOrNumber::P(p) = &mut *self.left {
            let (was_exploding, destroy_child, retleft, mut retright) = p.explode_inner(level + 1);
            if let Some(n) = retright {
                let mut p = &mut *self.right;
                while p.is_pair() {
                    p = &mut *p.unwrap_pair_mut().left;
                }
                *p.unwrap_number_mut() += n;
                retright = None;
            }

            if destroy_child {
                self.left = Box::new(PairOrNumber::N(0));
            }

            if was_exploding {
                return (true, false, retleft, retright);
            }
        }

        if let PairOrNumber::P(p) = &mut *self.right {
            let (was_exploding, destroy_child, mut retleft, retright) = p.explode_inner(level + 1);
            if let Some(n) = retleft {
                let mut p = &mut *self.left;
                while p.is_pair() {
                    p = &mut *p.unwrap_pair_mut().right;
                }
                *p.unwrap_number_mut() += n;
                retleft = None;
            }

            if destroy_child {
                self.right = Box::new(PairOrNumber::N(0));
            }

            if was_exploding {
                return (true, false, retleft, retright);
            }
        }

        (false, false, None, None)
    }

    fn from_split(n: u64) -> Self {
        let leftval = n / 2;
        let rightval = n - leftval;
        Pair {
            left: Box::new(PairOrNumber::N(leftval)),
            right: Box::new(PairOrNumber::N(rightval)),
        }
    }

    fn split(&mut self) -> bool {
        match &mut *self.left {
            PairOrNumber::N(n) => {
                if *n >= 10 {
                    self.left = Box::new(PairOrNumber::P(Self::from_split(*n)));
                    return true;
                }
            }
            PairOrNumber::P(p) => {
                if p.split() {
                    return true;
                }
            }
        }

        match &mut *self.right {
            PairOrNumber::N(n) => {
                if *n >= 10 {
                    self.right = Box::new(PairOrNumber::P(Self::from_split(*n)));
                    return true;
                }
            }
            PairOrNumber::P(p) => {
                if p.split() {
                    return true;
                }
            }
        }

        false
    }
}

fn nested_list_split(s: &str) -> NestedListIter {
    let mut s_chars = s.chars();
    let a = s_chars.next().unwrap();
    assert_eq!(a, '[');

    let b = s_chars.next_back().unwrap();
    assert_eq!(b, ']');

    NestedListIter {
        rest: s_chars.as_str(),
    }
}

struct NestedListIter<'a> {
    rest: &'a str,
}

impl<'a> Iterator for NestedListIter<'a> {
    type Item = (&'a str, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }

        let mut bracket_counter = 0;
        let mut contains_bracket = false;
        for (i, c) in self.rest.char_indices() {
            match c {
                '[' => {
                    bracket_counter += 1;
                    contains_bracket = true;
                }
                ']' => {
                    assert!(bracket_counter > 0, "invalid input");
                    bracket_counter -= 1;
                }
                ',' => {
                    assert_ne!(i, 0, "invalid input");
                    if bracket_counter == 0 {
                        let result_inner = &self.rest[..i];
                        self.rest = &self.rest[(i + 1)..];
                        return Some((result_inner.trim(), contains_bracket));
                    }
                }
                _ => (),
            }
        }

        let result_inner = self.rest;
        self.rest = "";

        Some((result_inner.trim(), contains_bracket))
    }
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
