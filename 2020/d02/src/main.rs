fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(Debug)]
struct PasswordListItem {
    n_min_key: usize,
    n_max_key: usize,
    key: char,
    password: String,
}

impl PasswordListItem {
    fn new(n_min_key: usize, n_max_key: usize, key: char, password: String) -> Self {
        Self {
            n_min_key,
            n_max_key,
            key,
            password,
        }
    }

    fn is_valid_old_rule(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.key).count();
        count >= self.n_min_key && count <= self.n_max_key
    }

    fn is_valid_new_rule(&self) -> bool {
        let mut count = 0;
        for &i in [self.n_min_key, self.n_max_key].iter() {
            if self.password.chars().nth(i - 1).unwrap() == self.key {
                count += 1;
            }
        }
        count == 1
    }
}

fn parse_input(input: &str) -> Vec<PasswordListItem> {
    use regex::Regex;
    let re = Regex::new(r"(\d+?)-(\d+?)\s+?(\w):\s+?(\w*)").unwrap();
    let mut result = Vec::new();

    for cap in re.captures_iter(input) {
        result.push(PasswordListItem::new(
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].chars().nth(0).unwrap(),
            cap[4].to_owned(),
        ));
    }

    result
}

fn compute_solution_part_one(input: &str) -> usize {
    let password_list = parse_input(input);
    password_list
        .iter()
        .filter(|p| p.is_valid_old_rule())
        .count()
}

fn compute_solution_part_two(input: &str) -> usize {
    let password_list = parse_input(input);
    password_list
        .iter()
        .filter(|p| p.is_valid_new_rule())
        .count()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d02 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
