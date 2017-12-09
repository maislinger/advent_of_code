struct StreamParser {
    group_level: u64,
    score: u64,
    nr_garbage_chars: u64,
    ignore_next: bool,
    garbage_mode: bool,
}

impl StreamParser {
    fn new() -> StreamParser {
        StreamParser {
            group_level: 0,
            score: 0,
            nr_garbage_chars: 0,
            ignore_next: false,
            garbage_mode: false,
        }
    }

    fn parse_char(&mut self, input: &char) {
        if !self.ignore_next && !self.garbage_mode {
            match *input {
                '{' => self.group_level += 1,
                '}' => {
                    self.score += self.group_level;
                    self.group_level -= 1;
                }
                '<' => self.garbage_mode = true,
                _ => (),
            }
        } else if !self.ignore_next && self.garbage_mode {
            match *input {
                '!' => self.ignore_next = true,
                '>' => self.garbage_mode = true,
                _ => self.nr_garbage_chars += 1,
            }
            if *input == '>' {
                self.garbage_mode = false;
            }
        } else {
            self.ignore_next = false;
        }
    }

    fn parse_str(&mut self, input: &str) {
        input.chars().map(|c| self.parse_char(&c)).count();
    }
}

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );
    contents.trim().to_owned()
}

fn compute_solution_part_one(input: &str) -> u64 {
    let mut parser = StreamParser::new();
    parser.parse_str(input);
    parser.score
}

fn compute_solution_part_two(input: &str) -> u64 {
    let mut parser = StreamParser::new();
    parser.parse_str(input);
    parser.nr_garbage_chars
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        let mut examples = Vec::new();
        examples.push(("{}".to_owned(), 1));
        examples.push(("{{{}}}".to_owned(), 6));
        examples.push(("{{},{}}".to_owned(), 5));
        examples.push(("{{{},{},{{}}}}".to_owned(), 16));
        examples.push(("{<a>,<a>,<a>,<a>}".to_owned(), 1));
        examples.push(("{{<ab>},{<ab>},{<ab>},{<ab>}}".to_owned(), 9));
        examples.push(("{{<!!>},{<!!>},{<!!>},{<!!>}}".to_owned(), 9));
        examples.push(("{{<a!>},{<a!>},{<a!>},{<ab>}}".to_owned(), 3));
        examples
            .iter()
            .map(|&(ref input, value)| {
                let solution = compute_solution_part_one(&input);
                assert_eq!(solution, value);
            })
            .count();

        let mut examples = Vec::new();
        examples.push(("<>".to_owned(), 0));
        examples.push(("<random characters>".to_owned(), 17));
        examples.push(("<<<<>".to_owned(), 3));
        examples.push(("<{!>}>".to_owned(), 2));
        examples.push(("<!!>".to_owned(), 0));
        examples.push(("<!!!>>".to_owned(), 0));
        examples.push(("<{o\"i!a,<{i<a>".to_owned(), 10));
        examples
            .iter()
            .map(|&(ref input, value)| {
                let solution = compute_solution_part_two(&input);
                assert_eq!(solution, value);
            })
            .count();
    }
}


fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d09 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
