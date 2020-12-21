use std::collections::{BTreeMap, BTreeSet};

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> Vec<(BTreeSet<String>, BTreeSet<String>)> {
    let mut result = Vec::new();

    for line in input.lines() {
        let ingredients: BTreeSet<String> = line
            .split('(')
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let allergens: BTreeSet<String> = line
            .split("contains ")
            .nth(1)
            .unwrap()
            .split(')')
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        result.push((ingredients, allergens));
    }

    result
}

fn compute_solution_part_one(input: &str) -> usize {
    let list_of_foods = parse_input(input);
    let (allergens, ingredients) = {
        let mut allergens = BTreeSet::new();
        let mut ingredients = BTreeSet::new();
        for (i, a) in list_of_foods.iter() {
            allergens = allergens.union(a).cloned().collect();
            ingredients = ingredients.union(i).cloned().collect();
        }
        (allergens, ingredients)
    };

    let mut candidates: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for allergen in allergens.iter() {
        candidates.insert(allergen.clone(), ingredients.clone());
    }

    for (i, a_set) in list_of_foods.iter() {
        for a in a_set.iter() {
            candidates.insert(a.clone(), candidates[a].intersection(i).cloned().collect());
        }
    }

    let may_have_allergens = {
        let mut may_have_allergens: BTreeSet<String> = BTreeSet::new();
        for (_, i) in candidates.iter() {
            may_have_allergens = may_have_allergens.union(i).cloned().collect();
        }
        may_have_allergens
    };

    let contains_no_allergens: BTreeSet<String> = ingredients
        .difference(&may_have_allergens)
        .cloned()
        .collect();

    let mut count = 0;
    for (i, _) in list_of_foods.iter() {
        count += i.intersection(&contains_no_allergens).count()
    }

    count
}

fn compute_solution_part_two(input: &str) -> String {
    let list_of_foods = parse_input(input);
    let (allergens, ingredients) = {
        let mut allergens = BTreeSet::new();
        let mut ingredients = BTreeSet::new();
        for (i, a) in list_of_foods.iter() {
            allergens = allergens.union(a).cloned().collect();
            ingredients = ingredients.union(i).cloned().collect();
        }
        (allergens, ingredients)
    };

    let mut candidates: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for allergen in allergens.iter() {
        candidates.insert(allergen.clone(), ingredients.clone());
    }

    for (i, a_set) in list_of_foods.iter() {
        for a in a_set.iter() {
            candidates.insert(a.clone(), candidates[a].intersection(i).cloned().collect());
        }
    }

    let mut assigned: BTreeSet<String> = BTreeSet::new();
    let mut changed = true;

    while changed {
        changed = false;
        let tmp = candidates
            .iter()
            .find(|(_, i_set)| i_set.len() == 1 && !assigned.contains(i_set.iter().next().unwrap()))
            .map(|(a, i)| (a.clone(), i.iter().cloned().next().unwrap()));
        if let Some((a0, i0)) = tmp {
            assigned.insert(i0.clone());
            changed = true;
            for (a, i) in candidates.iter_mut() {
                if *a == a0 {
                    continue;
                }
                i.remove(&i0);
            }
        }
    }

    // btreemap is sorted
    let mut result: String = candidates
        .iter()
        .map(|(_, i)| i.iter().next().unwrap().to_string())
        .next()
        .unwrap();

    for (_, i) in candidates.iter().skip(1) {
        result.push_str(",");
        result.push_str(i.iter().next().unwrap());
    }

    result
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d21 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
