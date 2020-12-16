#[macro_use]
extern crate itertools;

use regex::Regex;
use std::collections::{HashMap, HashSet};

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

struct TicketInfo {
    field_descriptors: HashMap<String, Vec<(usize, usize)>>,
    my_ticket: Vec<usize>,
    other_tickets: Vec<Vec<usize>>,
}

impl TicketInfo {
    fn new(
        field_descriptors: HashMap<String, Vec<(usize, usize)>>,
        my_ticket: Vec<usize>,
        other_tickets: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            field_descriptors,
            my_ticket,
            other_tickets,
        }
    }

    fn in_any_range(&self, value: usize) -> bool {
        for range in self.field_descriptors.iter().flat_map(|(_, v)| v.iter()) {
            if value >= range.0 && value <= range.1 {
                return true;
            }
        }
        false
    }

    fn remove_invalid_tickets(&mut self) {
        let mut keep = Vec::new();
        for ticket in self.other_tickets.iter() {
            let mut valid = true;
            for value in ticket.iter() {
                if !self.in_any_range(*value) {
                    valid = false;
                    break;
                }
            }
            keep.push(valid);
        }

        let kept_tickets = self
            .other_tickets
            .iter()
            .zip(keep.iter())
            .filter(|(_, k)| **k)
            .map(|(v, _)| v.clone())
            .collect();
        self.other_tickets = kept_tickets;
    }
}

impl Default for TicketInfo {
    fn default() -> Self {
        Self::new(HashMap::new(), Vec::new(), Vec::new())
    }
}

fn parse_input(input: &str) -> TicketInfo {
    let mut ticket_info = TicketInfo::default();

    let re_field_name = Regex::new(r"(.*):").unwrap();
    let re_ranges = Regex::new(r"(\d*)-(\d*)").unwrap();
    for line in input.lines() {
        if line.contains("your ticket") {
            break;
        }
        if !re_field_name.is_match(line) {
            continue;
        }
        let field_name = re_field_name.captures_iter(line).next().unwrap()[1].to_string();
        let mut ranges = Vec::new();
        for cap in re_ranges.captures_iter(line) {
            let lower: usize = cap[1].parse().unwrap();
            let upper: usize = cap[2].parse().unwrap();
            ranges.push((lower, upper));
        }
        ticket_info.field_descriptors.insert(field_name, ranges);
    }

    let mut read = false;
    for line in input.lines() {
        if line.contains("your ticket") {
            read = true;
            continue;
        }
        if !read {
            continue;
        }
        for n in line.split(',') {
            ticket_info.my_ticket.push(n.parse().unwrap());
        }
        break;
    }

    let mut read = false;
    for line in input.lines() {
        if line.contains("nearby tickets") {
            read = true;
            continue;
        }
        if !read {
            continue;
        }
        let mut new_ticket = Vec::new();
        for n in line.split(',') {
            new_ticket.push(n.parse().unwrap());
        }
        ticket_info.other_tickets.push(new_ticket);
    }
    let n = ticket_info.my_ticket.len();
    for other_ticket in ticket_info.other_tickets.iter() {
        assert_eq!(n, other_ticket.len());
    }
    ticket_info
}

fn compute_solution_part_one(input: &str) -> usize {
    let ticket_info = parse_input(input);
    let mut result = 0;
    for value in ticket_info.other_tickets.iter().flat_map(|t| t.iter()) {
        if !ticket_info.in_any_range(*value) {
            result += value;
        }
    }
    result
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut ticket_info = parse_input(input);
    ticket_info.remove_invalid_tickets();
    let n_cols = ticket_info.my_ticket.len();
    let n_rows = ticket_info.other_tickets.len();
    let mut candidates: Vec<HashSet<String>> = vec![
        ticket_info
            .field_descriptors
            .iter()
            .map(|(k, _)| k.to_string())
            .collect();
        n_cols
    ];

    for (i, j) in iproduct!(0..n_rows, 0..n_cols) {
        let value = ticket_info.other_tickets[i][j];
        for (k, v) in ticket_info.field_descriptors.iter() {
            let mut valid = false;
            for ranges in v.iter() {
                if value >= ranges.0 && value <= ranges.1 {
                    valid = true;
                }
            }
            if !valid {
                candidates[j].remove(k);
            }
        }
    }

    let mut assigned = HashSet::new();

    loop {
        let mut next_index = None;
        for (i, c) in candidates.iter().enumerate() {
            if c.len() > 1 {
                continue;
            }
            if c.len() == 1 && !assigned.contains(c.iter().next().unwrap()) {
                next_index = Some(i);
            }
        }

        if next_index.is_none() {
            break;
        }

        let next_index = next_index.unwrap();
        let candidate = candidates[next_index].iter().next().unwrap().clone();
        assigned.insert(candidate.to_string());
        for i in 0..n_cols {
            if i != next_index {
                candidates[i].remove(&candidate as &str);
            }
        }
    }

    for c in candidates.iter() {
        assert_eq!(c.len(), 1, "no valid configuration found");
    }
    let candidates: Vec<String> = candidates
        .iter()
        .map(|c| c.iter().next().unwrap().clone())
        .collect();
    let mut result = 1;

    for (i, _) in candidates
        .iter()
        .enumerate()
        .filter(|(_, c)| c.starts_with("departure"))
    {
        result *= ticket_info.my_ticket[i];
    }

    result
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d16 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
