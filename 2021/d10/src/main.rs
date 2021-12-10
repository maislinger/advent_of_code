fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d10 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut result = 0;

    for line in input.lines() {
        let r = check_line(line);
        if let CheckLineResult::Corrupt(c) = r {
            result += points_corrupt(c);
        }
    }

    result
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut points = Vec::new();

    for line in input.lines() {
        let r = check_line(line);
        if let CheckLineResult::Incomplete(mut s) = r {
            s.reverse();
            points.push(
                s.iter()
                    .map(|b| inverse_bracket(*b))
                    .map(points_autocomplete)
                    .fold(0, |acc, n| acc * 5 + n),
            );
        }
    }

    assert_eq!(
        points.len() % 2,
        1,
        "The points do not have a middle element"
    );
    let median_index = points.len() / 2;
    points.select_nth_unstable(median_index);
    points[median_index]
}

fn check_line(line: &str) -> CheckLineResult {
    let mut state = Vec::new();
    let mut breaking_char = None;
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => state.push(c),
            ')' | ']' | '}' | '>' => {
                if state.is_empty() {
                    breaking_char = Some(c);
                    break;
                }

                let last = state.pop().unwrap();
                if last != inverse_bracket(c) {
                    breaking_char = Some(c);
                    break;
                }
            }
            _ => panic!("invalid input."),
        }
    }

    if let Some(c) = breaking_char {
        CheckLineResult::Corrupt(c)
    } else if state.is_empty() {
        CheckLineResult::Complete
    } else {
        CheckLineResult::Incomplete(state)
    }
}

enum CheckLineResult {
    Complete,              // all brackets match
    Incomplete(Vec<char>), // brackets to be closed
    Corrupt(char),         // breaking braket
}

fn points_corrupt(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid input."),
    }
}

fn points_autocomplete(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("invalid input: {}", c),
    }
}

fn inverse_bracket(c: char) -> char {
    match c {
        ')' => '(',
        '(' => ')',
        ']' => '[',
        '[' => ']',
        '}' => '{',
        '{' => '}',
        '>' => '<',
        '<' => '>',
        _ => panic!("invalid input: {}", c),
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
