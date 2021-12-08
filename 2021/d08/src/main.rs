fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn compute_solution_part_one(input: &str) -> usize {
    let mut sum_1478 = 0;
    for line in input.lines() {
        let digits = line.split(" | ").nth(1).unwrap();
        sum_1478 += digits
            .split_whitespace()
            .map(|d| d.chars().count())
            .filter(|&c| c == 2 || c == 4 || c == 3 || c == 7)
            .count();
    }

    sum_1478
}

#[derive(Copy, Clone)]
struct SegmentConfiguration {
    active: [bool; 7],
}

impl SegmentConfiguration {
    // Used for unused segments.
    // A valid digit can never have all segments inactive.
    fn zeros() -> Self {
        Self { active: [false; 7] }
    }

    fn from_str(s: &str) -> Self {
        let mut result = Self::zeros();
        for c in s.chars() {
            match c {
                'a' => result.active[0] = true,
                'b' => result.active[1] = true,
                'c' => result.active[2] = true,
                'd' => result.active[3] = true,
                'e' => result.active[4] = true,
                'f' => result.active[5] = true,
                'g' => result.active[6] = true,
                _ => panic!("invalid segment configuration"),
            }
        }
        result
    }

    fn count_active(&self) -> usize {
        self.active.iter().filter(|s| **s).count()
    }

    fn n_different_segments(&self, other: &Self) -> usize {
        let mut result = 0;
        for i in 0..7 {
            if self.active[i] != other.active[i] {
                result += 1;
            }
        }
        result
    }

    fn same_configuration(&self, other: &Self) -> bool {
        self.n_different_segments(other) == 0
    }
}

fn get_entry_value(entry: &str) -> usize {
    let mut samples = [SegmentConfiguration::zeros(); 10];
    let mut digits = [SegmentConfiguration::zeros(); 4];
    let samples_iter = entry.split(" | ").next().unwrap().split_whitespace();
    let digits_iter = entry.split(" | ").nth(1).unwrap().split_whitespace();

    for (i, s) in samples_iter.enumerate() {
        samples[i] = SegmentConfiguration::from_str(s);
    }

    for (i, s) in digits_iter.enumerate() {
        digits[i] = SegmentConfiguration::from_str(s);
    }

    // All digits assigned?
    assert_eq!(samples.iter().filter(|s| s.count_active() != 0).count(), 10);
    assert_eq!(digits.iter().filter(|s| s.count_active() != 0).count(), 4);

    let mut identified_digits = [SegmentConfiguration::zeros(); 10];

    for s in samples.iter_mut() {
        let active = s.count_active();
        let index = match active {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        };

        if let Some(j) = index {
            identified_digits[j] = *s;
            *s = SegmentConfiguration::zeros();
        }
    }

    for s in samples.iter_mut() {
        if s.count_active() == 0 {
            continue;
        }
        let d1 = s.n_different_segments(&identified_digits[1]);
        let d4 = s.n_different_segments(&identified_digits[4]);
        let d7 = s.n_different_segments(&identified_digits[7]);
        let d8 = s.n_different_segments(&identified_digits[8]);

        let i = match (d1, d4, d7, d8) {
            (4, 4, 3, 1) => 0,
            (5, 5, 4, 2) => 2,
            (3, 3, 2, 2) => 3,
            (5, 3, 4, 2) => 5,
            (6, 4, 5, 1) => 6,
            (1, 3, 0, 4) => 7,
            (4, 2, 3, 1) => 9,
            _ => panic!("invalid 7-segment digit"),
        };

        identified_digits[i] = *s;
        *s = SegmentConfiguration::zeros();
    }

    // All digits found?
    assert_eq!(samples.iter().filter(|s| s.count_active() != 0).count(), 0);

    let mut result = 0;
    for d in digits.iter() {
        result *= 10;
        for (i, s) in identified_digits.iter().enumerate() {
            if d.same_configuration(s) {
                result += i;
                break;
            }
        }
    }

    result
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        result += get_entry_value(line);
    }
    result
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d08 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
