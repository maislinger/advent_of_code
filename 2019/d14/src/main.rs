use std::collections::{BTreeMap, VecDeque};

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_info(info: &str) -> (i64, String) {
    let mut tmp = info.trim().split(' ');
    let nr = tmp.next().unwrap().parse().unwrap();
    let compound = tmp.next().unwrap().to_string();
    (nr, compound)
}

struct ProductInfo {
    product_coefficient: i64,
    coefficients: Vec<i64>,
    chemicals: Vec<String>,
}

impl ProductInfo {
    fn new(product_coefficient: i64, coefficients: Vec<i64>, chemicals: Vec<String>) -> Self {
        Self {
            product_coefficient,
            coefficients,
            chemicals,
        }
    }

    fn factor(&self, needed_quantity: i64) -> i64 {
        if needed_quantity % self.product_coefficient == 0 {
            needed_quantity / self.product_coefficient
        } else {
            needed_quantity / self.product_coefficient + 1
        }
    }

    fn actual_quantity(&self, needed_quantity: i64) -> i64 {
        self.product_coefficient * self.factor(needed_quantity)
    }

    fn iter(&self, needed_quantity: i64) -> ProductInfoIterator {
        ProductInfoIterator::new(
            self.factor(needed_quantity),
            self.coefficients.clone(),
            self.chemicals.clone(),
        )
    }
}

struct ProductInfoIterator {
    factor: i64,
    coefficients: Vec<i64>,
    chemicals: Vec<String>,
    index: usize,
}

impl ProductInfoIterator {
    fn new(factor: i64, coefficients: Vec<i64>, chemicals: Vec<String>) -> Self {
        Self {
            factor,
            coefficients,
            chemicals,
            index: 0,
        }
    }
}

impl Iterator for ProductInfoIterator {
    type Item = (i64, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.chemicals.len() {
            self.index += 1;
            let i = self.index - 1;
            Some((
                self.coefficients[i] * self.factor,
                self.chemicals[i].clone(),
            ))
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> BTreeMap<String, ProductInfo> {
    let mut result = BTreeMap::new();
    for line in input.lines() {
        let reactants = line.split("=>").nth(0).unwrap();
        let product = line.split("=>").nth(1).unwrap();
        let product = parse_info(product);
        let reactants: Vec<_> = reactants.split(',').map(|s| parse_info(s)).collect();
        let coefficients: Vec<_> = reactants.iter().map(|r| r.0).collect();
        let chemicals: Vec<_> = reactants.iter().map(|r| r.1.clone()).collect();
        let product_info = ProductInfo::new(product.0, coefficients, chemicals);
        result.insert(product.1, product_info);
    }
    result
}

fn ore_for_fuel(fuel: i64, info_table: &BTreeMap<String, ProductInfo>) -> i64 {
    let mut chemicals = BTreeMap::new();
    chemicals.insert("FUEL".to_string(), fuel);
    let mut todo = VecDeque::new();
    todo.push_back("FUEL".to_string());

    while !todo.is_empty() {
        let next_chemical = todo.pop_front().unwrap();
        let needed_quantity = chemicals[&next_chemical];
        let actual_quantity = info_table[&next_chemical].actual_quantity(needed_quantity);
        chemicals.insert(next_chemical.clone(), needed_quantity - actual_quantity);

        for (nr, chemical) in info_table[&next_chemical].iter(needed_quantity) {
            *chemicals.entry(chemical.clone()).or_insert(0) += nr;
            if chemical != "ORE" && !todo.iter().any(|t| *t == chemical) {
                todo.push_back(chemical);
            }
        }
    }

    chemicals["ORE"]
}

fn compute_solution_part_one(input: &str) -> i64 {
    let info_table = parse_input(input);
    ore_for_fuel(1, &info_table)
}

fn compute_solution_part_two(input: &str) -> i64 {
    let info_table = parse_input(input);
    let available_ore: i64 = 1_000_000_000_000;
    let mut lower_fuel = 1;
    let mut lower_ore = ore_for_fuel(lower_fuel, &info_table);
    let mut upper_fuel = 2;
    let mut upper_ore = ore_for_fuel(upper_fuel, &info_table);
    let mut middle_fuel;
    let mut middle_ore;

    while upper_ore < available_ore {
        lower_ore = upper_ore;
        lower_fuel = upper_fuel;
        upper_fuel *= 2;
        upper_ore = ore_for_fuel(upper_fuel, &info_table);
    }

    loop {
        assert!(available_ore > lower_ore);
        assert!(available_ore < upper_ore);
        middle_fuel = (lower_fuel + upper_fuel) / 2;
        if middle_fuel == lower_fuel {
            middle_fuel += 1;
        }
        if middle_fuel == upper_fuel {
            middle_fuel -= 1;
        }
        middle_ore = ore_for_fuel(middle_fuel, &info_table);

        if middle_ore == lower_ore || middle_ore == available_ore {
            break;
        }

        if middle_ore > available_ore {
            upper_fuel = middle_fuel;
            upper_ore = middle_ore;
        } else if middle_ore < available_ore {
            lower_fuel = middle_fuel;
            lower_ore = middle_ore;
        }
    }

    middle_fuel
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d14 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
