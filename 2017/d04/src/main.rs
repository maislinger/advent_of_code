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

fn check_valid_duplicate(input: &str) -> bool {
    use std::iter::FromIterator;
    use std::collections::BTreeSet;

    let nr_words = input.split_whitespace().count();
    let btree_set = BTreeSet::from_iter(input.split_whitespace());
    let nr_unique_words = btree_set.len();
    nr_words == nr_unique_words
}

fn check_valid_anagram(input: &str) -> bool {
    use std::iter::FromIterator;
    use std::collections::BTreeSet;

    let nr_words = input.split_whitespace().count();
    let iter = input.split_whitespace().map(|w| {
        let mut chars: Vec<char> = w.chars().collect();
        chars.sort();
        String::from_iter(chars.iter())
    });
    let btree_set = BTreeSet::from_iter(iter);
    let nr_unique_words = btree_set.len();
    nr_words == nr_unique_words
}

fn compute_solution_part_one(input: &str) -> u64 {
    input
        .split('\n')
        .filter(|l| check_valid_duplicate(l))
        .count() as u64
}

fn compute_solution_part_two(input: &str) -> u64 {
    input.split('\n').filter(|l| check_valid_anagram(l)).count() as u64
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        assert_eq!(compute_solution_part_one("aa bb cc dd ee"), 1);
        assert_eq!(compute_solution_part_one("aa bb cc dd aa"), 0);
        assert_eq!(compute_solution_part_one("aa bb cc dd aaa"), 1);
        assert_eq!(
            compute_solution_part_one("aa bb cc dd ee\naa bb cc dd aa"),
            1
        );

        assert_eq!(compute_solution_part_two("abcde fghij"), 1);
        assert_eq!(compute_solution_part_two("abcde xyz ecdab"), 0);
        assert_eq!(compute_solution_part_two("a ab abc abd abf abj"), 1);
        assert_eq!(compute_solution_part_two("iiii oiii ooii oooi oooo"), 1);
        assert_eq!(compute_solution_part_two("oiii ioii iioi iiio"), 0);
        assert_eq!(
            compute_solution_part_two("abcde fghij\na ab abc abd abf abj"),
            2
        );
    }
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
