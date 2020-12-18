fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Symbol {
    Number(i64),
    Addition,
    Multiplication,
    OpenBracket,
    CloseBracket,
}

impl Symbol {
    fn unwrap_number(&self) -> i64 {
        match self {
            Symbol::Number(x) => *x,
            _ => panic!("tried to unwrap other symbol than number"),
        }
    }
}

fn parse_line(line: &str) -> Vec<Symbol> {
    let mut result = Vec::new();
    let mut current_number = None;

    for c in line.chars().filter(|c| *c != ' ') {
        match c {
            '+' | '*' | '(' | ')' => {
                if current_number.is_some() {
                    result.push(Symbol::Number(current_number.unwrap()));
                    current_number = None;
                }
            }
            _ => (),
        }
        match c {
            '0'..='9' => {
                if current_number.is_none() {
                    current_number = Some(0);
                }
                current_number = current_number.map(|n| n * 10 + (c.to_digit(10).unwrap() as i64));
            }
            '+' => result.push(Symbol::Addition),
            '*' => result.push(Symbol::Multiplication),
            '(' => result.push(Symbol::OpenBracket),
            ')' => result.push(Symbol::CloseBracket),
            _ => panic!("invalid symbol in expression"),
        }
    }
    if current_number.is_some() {
        result.push(Symbol::Number(current_number.unwrap()));
    }
    result
}

fn contains_brackets(e: &[Symbol]) -> bool {
    e.iter()
        .filter(|s| **s == Symbol::OpenBracket || **s == Symbol::CloseBracket)
        .count()
        != 0
}

fn parse_expression_no_brackets(e: &[Symbol]) -> i64 {
    let mut result = e[0].unwrap_number();
    if e.len() == 1 {
        return result;
    }

    for (n, s) in e
        .iter()
        .skip(2)
        .step_by(2)
        .map(|s| s.unwrap_number())
        .zip(e.iter().skip(1).step_by(2))
    {
        match s {
            Symbol::Addition => result += n,
            Symbol::Multiplication => result *= n,
            _ => panic!("invalid operation"),
        }
    }

    result
}

fn parse_expression_no_brackets_addition_first(e: &[Symbol]) -> i64 {
    let mut e = e.to_vec();
    while e.iter().filter(|s| **s == Symbol::Addition).count() > 0 {
        let i = e
            .iter()
            .enumerate()
            .find(|(_, s)| **s == Symbol::Addition)
            .unwrap()
            .0;
        let m = e[i - 1].unwrap_number();
        let n = e[i + 1].unwrap_number();
        let r = m + n;
        e[i - 1] = Symbol::Number(r);
        let tmp_vec = e.split_off(i);
        e.extend_from_slice(&tmp_vec[2..]);
    }
    parse_expression_no_brackets(&e)
}

fn inner_bracket_indices(e: &[Symbol]) -> (usize, usize) {
    let mut first_bracket = None;
    for i in 0..e.len() {
        match e[i] {
            Symbol::OpenBracket => first_bracket = Some(i),
            Symbol::CloseBracket => {
                assert!(first_bracket.is_some());
                return (first_bracket.unwrap(), i);
            }
            _ => (),
        }
    }
    panic!("no brackets found");
}

fn parse_expression(e: &[Symbol], addition_first: bool) -> i64 {
    let mut e: Vec<Symbol> = e.to_vec();
    while contains_brackets(&e) {
        let (i, j) = inner_bracket_indices(&e);
        let n = if !addition_first {
            parse_expression_no_brackets(&e[(i + 1)..j])
        } else {
            parse_expression_no_brackets_addition_first(&e[(i + 1)..j])
        };
        e[i] = Symbol::Number(n);
        let tmp_vec = e.split_off(i + 1);
        e.extend_from_slice(&tmp_vec[(j - i)..])
    }
    if !addition_first {
        parse_expression_no_brackets(&e)
    } else {
        parse_expression_no_brackets_addition_first(&e)
    }
}

fn compute_solution_part_one(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let parsed_symbols = parse_line(line);
        sum += parse_expression(&parsed_symbols, false);
    }
    sum
}

fn compute_solution_part_two(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let parsed_symbols = parse_line(line);
        sum += parse_expression(&parsed_symbols, true);
    }
    sum
}

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
