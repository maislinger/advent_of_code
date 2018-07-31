extern crate regex;

enum SearchStatus {
    Complete,
    NotComplete,
}

struct Component {
    a: usize,
    b: usize,
}

impl Component {
    fn new(a: usize, b: usize) -> Self {
        Component { a, b }
    }

    fn sum(&self) -> usize {
        self.a + self.b
    }
}

struct Walker {
    components: Vec<Component>,
    used: Vec<bool>,
    path: Vec<Option<usize>>,
    endpins: Vec<Option<usize>>,
    pathsum: Vec<Option<usize>>,
    max: usize,
    level: usize,
    maxlevel: usize,
    max_at_maxlevel: usize,
}

impl Walker {
    fn new(components: Vec<Component>) -> Self {
        let used = vec![false; components.len()];
        let path = vec![None; components.len()];
        let endpins = vec![None; components.len()];
        let pathsum = vec![None; components.len()];
        let max = 0;
        let level = 0;
        let maxlevel = 0;
        let max_at_maxlevel = 0;
        Walker {
            components,
            used,
            path,
            endpins,
            pathsum,
            max,
            level,
            maxlevel,
            max_at_maxlevel,
        }
    }

    fn usable(&self) -> impl Iterator<Item = (usize, &Component)> {
        self.components
            .iter()
            .enumerate()
            .zip(self.used.iter())
            .filter(|&((_, _), u)| !u)
            .map(|((i, c), _)| (i, c))
            .filter(move |&(_, c)| {
                if self.level == 0 {
                    c.a == 0 || c.b == 0
                } else {
                    let pin = self.endpins[self.level - 1].unwrap();
                    c.a == pin || c.b == pin
                }
            })
    }

    fn next_pin(&self) -> Option<(usize, usize)> {
        self.usable()
            .map(|(i, c)| {
                if self.level == 0 && c.a == 0
                    || self.level != 0 && c.a == self.endpins[self.level - 1].unwrap()
                {
                    (i, c.b)
                } else {
                    (i, c.a)
                }
            }).filter(|&(_, n)| match self.endpins[self.level] {
                Some(endpin) => n < endpin,
                None => true,
            }).max_by_key(|&(_, n)| n)
    }

    fn step(&mut self) -> SearchStatus {
        let next_pin = self.next_pin();
        match (next_pin, self.level) {
            (Some((i, pin)), _) => {
                self.used[i] = true;
                self.path[self.level] = Some(i);
                self.endpins[self.level] = Some(pin);
                let sum = if self.level == 0 {
                    pin
                } else {
                    self.pathsum[self.level - 1].unwrap() + self.components[i].sum()
                };
                self.pathsum[self.level] = Some(sum);
                if self.max < sum {
                    self.max = sum;
                }
                self.level += 1;

                if self.level > self.maxlevel {
                    self.maxlevel = self.level;
                    self.max_at_maxlevel = sum;
                } else if self.level == self.maxlevel && self.max_at_maxlevel < sum {
                    self.max_at_maxlevel = sum;
                }

                SearchStatus::NotComplete
            }
            (None, 0) => SearchStatus::Complete,
            _ => {
                let i = self.path[self.level - 1].unwrap();
                self.used[i] = false;
                self.path[self.level] = None;
                self.endpins[self.level] = None;
                self.pathsum[self.level] = None;
                self.level -= 1;
                SearchStatus::NotComplete
            }
        }
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

fn convert_input(input: &str) -> Vec<Component> {
    use regex::Regex;

    let re: Regex = Regex::new(r"(\d+)/(\d+)").unwrap();
    let mut result = Vec::new();

    for cap in re.captures_iter(input) {
        let a: usize = cap[1].parse().unwrap();
        let b: usize = cap[2].parse().unwrap();
        let component = Component::new(a, b);
        result.push(component);
    }

    result
}

fn compute_solution_part_one(input: &str) -> usize {
    let components = convert_input(input);
    let mut walker = Walker::new(components);
    loop {
        if let SearchStatus::Complete = walker.step() {
            break;
        }
    }
    walker.max
}

fn compute_solution_part_two(input: &str) -> usize {
    let components = convert_input(input);
    let mut walker = Walker::new(components);
    loop {
        if let SearchStatus::Complete = walker.step() {
            break;
        }
    }
    walker.max_at_maxlevel
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let input = "0/2
                     2/2
                     2/3
                     3/4
                     3/5
                     0/1
                     10/1
                     9/10";
        let solution = compute_solution_part_one(input);
        assert_eq!(solution, 31);

        let solution = compute_solution_part_two(input);
        assert_eq!(solution, 19);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d24 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
