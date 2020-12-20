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
enum Rule {
    Match(char),
    SubRule(Vec<Vec<usize>>),
}

fn parse_rule(line: &str) -> (usize, Rule) {
    let n: usize = line.split(':').next().unwrap().parse().unwrap();
    if line.contains('\"') {
        if line.contains('a') {
            (n, Rule::Match('a'))
        } else {
            (n, Rule::Match('b'))
        }
    } else {
        let mut sub_rules = Vec::new();
        let parents = line.split(':').nth(1).unwrap();
        for sub_parents in parents.split('|') {
            let sub_rule: Vec<usize> = sub_parents
                .split_whitespace()
                .map(|m| m.parse().unwrap())
                .collect();
            sub_rules.push(sub_rule);
        }
        (n, Rule::SubRule(sub_rules))
    }
}

fn parse_input(input: &str) -> (BTreeMap<usize, Rule>, Vec<String>) {
    let mut rules = BTreeMap::new();
    for line in input.lines() {
        if line.chars().count() == 0 {
            break;
        }
        let (n, rule) = parse_rule(line);
        rules.insert(n, rule);
    }

    let mut messages = Vec::new();
    let mut read = false;
    for line in input.lines() {
        if line.chars().count() == 0 {
            read = true;
            continue;
        }
        if !read {
            continue;
        }
        messages.push(line.to_string());
    }

    (rules, messages)
}

fn is_valid_message(message: &str, rule_numbers: &[usize], rules: &BTreeMap<usize, Rule>) -> bool {
    if message.chars().count() == 0 && rule_numbers.is_empty() {
        return true;
    }
    if (message.chars().count() == 0 && !rule_numbers.is_empty())
        || (message.chars().count() > 0 && rule_numbers.is_empty())
    {
        return false;
    }

    match &rules[&rule_numbers[0]] {
        Rule::Match(c) => {
            if *c != message.chars().next().unwrap() {
                false
            } else {
                let new_message: String = message.chars().skip(1).collect();
                is_valid_message(&new_message, &rule_numbers[1..], rules)
            }
        }
        Rule::SubRule(v) => {
            let mut recursive_result = false;
            for subvec in v.iter() {
                let mut new_rule_numbers = subvec.clone();
                new_rule_numbers.extend_from_slice(&rule_numbers[1..]);
                recursive_result =
                    recursive_result || is_valid_message(message, &new_rule_numbers, rules);
            }
            recursive_result
        }
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let (rules, messages) = parse_input(input);
    messages
        .iter()
        .filter(|message| is_valid_message(message, &[0], &rules))
        .count()
}

fn compute_solution_part_two(input: &str) -> usize {
    let (mut rules, messages) = parse_input(input);
    rules.insert(8, Rule::SubRule(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::SubRule(vec![vec![42, 31], vec![42, 11, 31]]));
    messages
        .iter()
        .filter(|message| is_valid_message(message, &[0], &rules))
        .count()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d19 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
