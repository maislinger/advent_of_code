fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn compute_solution_part_one(input: &str) -> i64 {
    let mut positions: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();

    // edge cases
    if positions.len() <= 1 {
        return 0;
    } else if positions.len() == 2 {
        return (positions[1] - positions[0]).abs();
    }

    fn fuel_cost(x: i64, positions: &[i64]) -> i64 {
        positions.iter().map(|p| (p - x).abs()).sum()
    }

    let center = positions.len() / 2;
    positions.select_nth_unstable(center);
    let center_pos = positions[center];
    let center_fuel = fuel_cost(center_pos, &positions);

    if positions.len() % 2 == 1 {
        return center_fuel;
    }

    let center_left = center - 1;
    let center_left_pos = positions[center_left];
    let center_left_fuel = fuel_cost(center_left_pos, &positions);

    if center_fuel < center_left_fuel {
        center_fuel
    } else {
        center_left_fuel
    }
}

fn compute_solution_part_two(input: &str) -> i64 {
    let positions: Vec<i64> = input.split(',').map(|s| s.parse().unwrap()).collect();

    if positions.len() <= 1 {
        return 0;
    }

    let fuel_cost = |x: i64| -> i64 {
        positions
            .iter()
            .map(|p| {
                let d = (p - x).abs();
                d * (d + 1) / 2
            })
            .sum()
    };

    let almost_mean: i64 = positions.iter().sum::<i64>() / (positions.len() as i64);

    // Safety margin of three around the mean.
    // Should be more than enough
    ((almost_mean - 3)..=(almost_mean + 3))
        .map(fuel_cost)
        .min()
        .unwrap()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d07 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
