#[macro_use]
extern crate lazy_static;

use regex::Regex;
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

#[derive(Debug)]
struct Passport {
    entries: BTreeMap<String, String>,
}

impl Passport {
    fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.entries.insert(key.to_owned(), value.to_owned());
    }

    fn has_all_entries(&self) -> bool {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        for &required_field in required_fields.iter() {
            if !self.entries.contains_key(required_field) {
                return false;
            }
        }
        true
    }

    fn has_valid_year_field(&self, field: &str, lower: usize, upper: usize) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{4}$").unwrap();
        }

        let field_value = self.entries.get(field);
        if field_value.is_none() {
            return false;
        }
        let field_value = field_value.unwrap();
        if !RE.is_match(field_value) {
            return false;
        }

        let parsed: usize = field_value.parse().unwrap();
        parsed >= lower && parsed <= upper
    }

    fn has_valid_birth_year(&self) -> bool {
        self.has_valid_year_field("byr", 1920, 2002)
    }

    fn has_valid_issue_year(&self) -> bool {
        self.has_valid_year_field("iyr", 2010, 2020)
    }

    fn has_valid_expiration_year(&self) -> bool {
        self.has_valid_year_field("eyr", 2020, 2030)
    }

    fn has_valid_height(&self) -> bool {
        lazy_static! {
            static ref RECM: Regex = Regex::new(r"^(\d{3})cm$").unwrap();
            static ref REIN: Regex = Regex::new(r"^(\d{2})in$").unwrap();
        }
        let field_value = self.entries.get("hgt");
        if field_value.is_none() {
            return false;
        }
        let field_value = field_value.unwrap();
        if let Some(cap) = RECM.captures_iter(field_value).next() {
            let parsed: usize = cap[1].parse().unwrap();
            parsed >= 150 && parsed <= 193
        } else if let Some(cap) = REIN.captures_iter(field_value).next() {
            let parsed: usize = cap[1].parse().unwrap();
            parsed >= 59 && parsed <= 76
        } else {
            false
        }
    }

    fn has_valid_hair_color(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\#([0-9]|[a-f]){6}$").unwrap();
        }
        match self.entries.get("hcl") {
            None => false,
            Some(e) => RE.is_match(e),
        }
    }

    fn has_valid_eye_color(&self) -> bool {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
        }
        match self.entries.get("ecl") {
            None => false,
            Some(e) => RE.is_match(e),
        }
    }

    fn has_valid_passport_id(&self) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        }
        match self.entries.get("pid") {
            None => false,
            Some(e) => RE.is_match(e),
        }
    }

    fn has_valid_entries(&self) -> bool {
        self.has_valid_birth_year()
            && self.has_valid_issue_year()
            && self.has_valid_expiration_year()
            && self.has_valid_height()
            && self.has_valid_hair_color()
            && self.has_valid_eye_color()
            && self.has_valid_passport_id()
    }
}

fn parse_input(input: &str) -> Vec<Passport> {
    let mut result = Vec::new();
    for record in input.split("\n\n") {
        let mut passport = Passport::new();
        for entry in record.split_whitespace() {
            let mut split_entry = entry.split(':');
            let key = split_entry.next().unwrap();
            let value = split_entry.next().unwrap();
            passport.insert(key, value);
        }
        result.push(passport);
    }
    result
}

fn compute_solution_part_one(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter().filter(|p| p.has_all_entries()).count()
}

fn compute_solution_part_two(input: &str) -> usize {
    let passports = parse_input(input);
    passports.iter().filter(|p| p.has_valid_entries()).count()
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
