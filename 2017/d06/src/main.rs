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

fn convert_input(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn step(blocks: &mut Vec<u64>) {
    use std::cmp::Ordering;

    let (mut ind, mut rest_blocks) = {
        let tmp = blocks
            .iter()
            .enumerate()
            .max_by(|x, y| match x.1.cmp(y.1) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => y.0.cmp(&x.0),
            })
            .unwrap();
        (tmp.0, *tmp.1)
    };

    blocks[ind] = 0;
    while rest_blocks > 0 {
        ind = (ind + 1) % blocks.len();
        blocks[ind] += 1;
        rest_blocks -= 1;
    }
}

fn cycle_to_rep(blocks: &mut Vec<u64>) -> (u64, u64) {
    use std::collections::BTreeMap;
    let mut btree_set = BTreeMap::new();
    let mut steps = 0;

    let (steps, looplen) = loop {
        let oldval = btree_set.insert(blocks.clone(), steps);
        if let Some(old_step) = oldval {
            break (steps, steps - old_step)
        }

        step(blocks);
        steps += 1;
    };

    (steps, looplen)
}

fn compute_solution_part_one(input: &str) -> u64 {
    let mut blocks = convert_input(input);
    cycle_to_rep(&mut blocks).0
}

fn compute_solution_part_two(input: &str) -> u64 {
    let mut blocks = convert_input(input);
    cycle_to_rep(&mut blocks).1
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use compute_solution_part_one;
        use compute_solution_part_two;

        assert_eq!(compute_solution_part_one("0 2 7 0"), 5);
        assert_eq!(compute_solution_part_two("0 2 7 0"), 4);
    }
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d06 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
