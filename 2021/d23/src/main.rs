use std::collections::{BTreeMap, BTreeSet};

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d23 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    compute_energy(input)
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut s = "".to_owned();

    for line in input.lines().take(3) {
        s += line;
        s += "\n";
    }

    s += "  #D#C#B#A#\n  #D#B#A#C#";

    for line in input.lines().skip(3) {
        s += line;
        s += "\n";
    }

    compute_energy(&s)
}

fn compute_energy(input: &str) -> usize {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let (initial_burrow, layout) = parse_input(input);

    let mut cost = BTreeMap::new();
    cost.insert(initial_burrow.clone(), 0);

    let mut todo = BinaryHeap::new();
    todo.push(Reverse(EnergyEntry::new(
        initial_burrow.clone(),
        initial_burrow.min_cost(&layout),
    )));

    while !todo.is_empty() {
        let ec = todo.pop().unwrap();
        let b = ec.0.burrow;
        let e = cost[&b];

        let next_bs = b.all_reachable_borrows(e, &layout);

        for (k, v) in next_bs.iter() {
            let mc = k.min_cost(&layout);
            if mc == 0 {
                return *v;
            }

            let prev_cost = cost.get(k);
            if prev_cost.is_none() || prev_cost.unwrap() > v {
                cost.insert(k.clone(), *v);
                todo.push(Reverse(EnergyEntry::new(k.clone(), *v + mc)));
            }
        }
    }

    unreachable!();
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FieldContent {
    Empty,
    Amber,
    Bronze,
    Copper,
    Desert,
}

fn move_cost(c: FieldContent) -> usize {
    match c {
        FieldContent::Empty => panic!("empty fields cannot be moved"),
        FieldContent::Amber => 1,
        FieldContent::Bronze => 10,
        FieldContent::Copper => 100,
        FieldContent::Desert => 1000,
    }
}

struct BurrowLayout {
    connections: Vec<(usize, usize)>,
    min_distances: Matrix<usize>,
    index_amber: usize,
    index_bronze: usize,
    index_copper: usize,
    index_desert: usize,
    range_amber: [usize; 2],
    range_bronze: [usize; 2],
    range_copper: [usize; 2],
    range_desert: [usize; 2],
    non_stoppable: BTreeSet<usize>,
}

impl BurrowLayout {
    fn index_in_hallway(&self, i: usize) -> bool {
        i < self.index_amber
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Burrow {
    state: Vec<FieldContent>,
}

impl Burrow {
    fn swap(&mut self, i: usize, j: usize) {
        self.state.swap(i, j);
    }

    fn min_cost(&self, layout: &BurrowLayout) -> usize {
        let mut result = 0;

        let mut matched_amber = layout.index_amber;
        let mut matched_bronze = layout.index_bronze;
        let mut matched_copper = layout.index_copper;
        let mut matched_desert = layout.index_desert;

        let maybe_increase_index = |i: &mut usize, t: FieldContent, upper: usize| {
            while *i < upper && self.state[*i] == t {
                *i += 1;
            }
        };

        maybe_increase_index(
            &mut matched_amber,
            FieldContent::Amber,
            layout.range_amber[1],
        );
        maybe_increase_index(
            &mut matched_bronze,
            FieldContent::Bronze,
            layout.range_bronze[1],
        );
        maybe_increase_index(
            &mut matched_copper,
            FieldContent::Copper,
            layout.range_copper[1],
        );
        maybe_increase_index(
            &mut matched_desert,
            FieldContent::Desert,
            layout.range_desert[1],
        );

        for (i, &s) in self.state.iter().enumerate() {
            if s == FieldContent::Empty {
                continue;
            }

            let r = match s {
                FieldContent::Amber => &layout.range_amber,
                FieldContent::Bronze => &layout.range_bronze,
                FieldContent::Copper => &layout.range_copper,
                FieldContent::Desert => &layout.range_desert,
                FieldContent::Empty => unreachable!(),
            };

            if i >= r[0] && i < r[1] {
                continue;
            }

            let m = match s {
                FieldContent::Amber => &mut matched_amber,
                FieldContent::Bronze => &mut matched_bronze,
                FieldContent::Copper => &mut matched_copper,
                FieldContent::Desert => &mut matched_desert,
                FieldContent::Empty => unreachable!(),
            };

            let delta = move_cost(s);

            result += delta * layout.min_distances.get(i, *m);
            *m += 1;
            maybe_increase_index(m, s, r[1]);
        }

        result
    }

    fn all_reachable_borrows(
        &self,
        init_energy: usize,
        layout: &BurrowLayout,
    ) -> BTreeMap<Burrow, usize> {
        let mut result = BTreeMap::new();

        for i in 0..self.state.len() {
            if self.state[i] != FieldContent::Empty {
                let mut r = self.all_reachable_borrows_from_field(i, init_energy, layout);
                result.append(&mut r);
            }
        }

        result
    }

    fn all_reachable_borrows_from_field(
        &self,
        index: usize,
        init_energy: usize,
        layout: &BurrowLayout,
    ) -> BTreeMap<Burrow, usize> {
        let delta = move_cost(self.state[index]);
        let original_type = self.state[index];

        let room_range = match original_type {
            FieldContent::Amber => &layout.range_amber,
            FieldContent::Bronze => &layout.range_bronze,
            FieldContent::Copper => &layout.range_copper,
            FieldContent::Desert => &layout.range_desert,
            FieldContent::Empty => panic!("empty fields cannot be moved"),
        };

        let mut intermediate_result = BTreeMap::new();

        let starts_from_hallway = layout.index_in_hallway(index);

        let mut todo = vec![(self.clone(), init_energy, index)];

        while !todo.is_empty() {
            let (b, e, i) = todo.pop().unwrap();

            for n in neighbors(i, layout) {
                if b.state[n] == FieldContent::Empty {
                    let mut new_b = b.clone();
                    new_b.swap(i, n);
                    let new_energy = e + delta;

                    if new_b != *self {
                        let (energy_in_result, _) = intermediate_result
                            .entry(new_b.clone())
                            .or_insert((new_energy + 1, n));
                        if *energy_in_result > new_energy {
                            *energy_in_result = new_energy;
                            todo.push((new_b, new_energy, n));
                        }
                    }
                }
            }
        }

        intermediate_result.retain(|_, (_, i)| !layout.non_stoppable.contains(i));

        if starts_from_hallway {
            intermediate_result.retain(|_, (_, i)| !layout.index_in_hallway(*i));
        }

        intermediate_result.retain(|b, (_, i)| {
            if !layout.index_in_hallway(*i) {
                let r = room_range[0] <= *i && *i < room_range[1];
                let mut s = true;
                for j in room_range[0]..room_range[1] {
                    let t = b.state[j] == FieldContent::Empty || b.state[j] == original_type;
                    s = s && t;
                }
                r && s
            } else {
                true
            }
        });

        intermediate_result
            .iter()
            .map(|(b, (e, _))| (b.clone(), *e))
            .collect()
    }
}

fn parse_input(input: &str) -> (Burrow, BurrowLayout) {
    let mut state = Vec::new();
    let hallway_line = input.lines().nth(1).unwrap();

    for c in hallway_line.chars() {
        match c {
            'A' => state.push(FieldContent::Amber),
            'B' => state.push(FieldContent::Bronze),
            'C' => state.push(FieldContent::Copper),
            'D' => state.push(FieldContent::Desert),
            '.' => state.push(FieldContent::Empty),
            _ => (),
        }
    }

    let mut state_amber = Vec::new();
    let mut state_bronze = Vec::new();
    let mut state_copper = Vec::new();
    let mut state_desert = Vec::new();

    for line in input.lines().skip(2) {
        let mut i = 0;
        for c in line.chars() {
            let s = match i {
                0 => &mut state_amber,
                1 => &mut state_bronze,
                2 => &mut state_copper,
                3 => &mut state_desert,
                _ => panic!("parse error"),
            };

            let orglen = s.len();

            match c {
                'A' => s.push(FieldContent::Amber),
                'B' => s.push(FieldContent::Bronze),
                'C' => s.push(FieldContent::Copper),
                'D' => s.push(FieldContent::Desert),
                '.' => s.push(FieldContent::Empty),
                _ => (),
            }

            if orglen != s.len() {
                i = (i + 1) % 4;
            }
        }
    }

    assert_eq!(state_amber.len(), state_bronze.len());
    assert_eq!(state_amber.len(), state_copper.len());
    assert_eq!(state_amber.len(), state_desert.len());

    let room_size = state_amber.len();
    let index_amber = state.len();
    state.append(&mut state_amber);
    let index_bronze = state.len();
    state.append(&mut state_bronze);
    let index_copper = state.len();
    state.append(&mut state_copper);
    let index_desert = state.len();
    state.append(&mut state_desert);
    let mut connections = Vec::new();

    for i in 0..(index_amber - 1) {
        connections.push((i, i + 1));
    }

    for &(hw_connection, index) in &[
        (2, index_amber),
        (4, index_bronze),
        (6, index_copper),
        (8, index_desert),
    ] {
        connections.push((hw_connection, index));
        for i in index..(index + room_size - 1) {
            connections.push((i, i + 1));
        }
    }

    let mut non_stoppable = BTreeSet::new();
    non_stoppable.insert(2);
    non_stoppable.insert(4);
    non_stoppable.insert(6);
    non_stoppable.insert(8);

    let mut layout = BurrowLayout {
        connections,
        min_distances: Matrix::constant(0, 0, 0),
        index_amber,
        index_bronze,
        index_copper,
        index_desert,
        range_amber: [index_amber, index_amber + room_size],
        range_bronze: [index_bronze, index_bronze + room_size],
        range_copper: [index_copper, index_copper + room_size],
        range_desert: [index_desert, index_desert + room_size],
        non_stoppable,
    };

    let min_distances = compute_min_distances(&layout);
    layout.min_distances = min_distances;

    let burrow = Burrow { state };

    (burrow, layout)
}

fn compute_min_distances(layout: &BurrowLayout) -> Matrix<usize> {
    let mut n = *layout.connections.iter().map(|(a, _)| a).max().unwrap();
    let m = *layout.connections.iter().map(|(_, a)| a).max().unwrap();

    if m > n {
        n = m;
    }

    let mut result = Matrix::constant(0, n + 1, n + 1);

    for i in 0..(n + 1) {
        let mut todo = vec![(i, 0)];
        let mut distances = BTreeMap::new();
        distances.insert(i, 0);

        while !todo.is_empty() {
            let (j, d) = todo.pop().unwrap();
            let new_dist = d + 1;
            for k in neighbors(j, layout) {
                let old_dist = distances.entry(k).or_insert(new_dist + 1);
                if *old_dist > new_dist {
                    *old_dist = new_dist;
                    todo.push((k, new_dist));
                }
            }
        }

        for (k, v) in distances.iter() {
            result.set(i, *k, *v);
        }
    }

    result
}

#[derive(Debug)]
struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    fn constant(c: T, width: usize, height: usize) -> Self
    where
        T: Clone,
    {
        Self {
            width,
            height,
            data: vec![c; width * height],
        }
    }

    fn get(&self, i: usize, j: usize) -> T
    where
        T: Copy,
    {
        assert!(i < self.height);
        assert!(j < self.width);
        self.data[i * self.width + j]
    }

    fn set(&mut self, i: usize, j: usize, new_val: T) {
        assert!(i < self.height);
        assert!(j < self.width);
        self.data[i * self.width + j] = new_val;
    }
}

fn neighbors(index: usize, layout: &BurrowLayout) -> ConnectionsIter {
    ConnectionsIter {
        burrow_index: index,
        index: 0,
        layout,
    }
}

struct ConnectionsIter<'a> {
    burrow_index: usize,
    index: usize,
    layout: &'a BurrowLayout,
}

impl<'a> Iterator for ConnectionsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        while self.index < self.layout.connections.len() {
            if self.layout.connections[self.index].0 == self.burrow_index {
                result = Some(self.layout.connections[self.index].1);
                self.index += 1;
                break;
            } else if self.layout.connections[self.index].1 == self.burrow_index {
                result = Some(self.layout.connections[self.index].0);
                self.index += 1;
                break;
            }
            self.index += 1;
        }

        result
    }
}

#[derive(Eq, PartialEq)]
struct EnergyEntry {
    energy: usize,
    burrow: Burrow,
}

impl EnergyEntry {
    fn new(burrow: Burrow, energy: usize) -> Self {
        Self { energy, burrow }
    }
}

impl PartialOrd for EnergyEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EnergyEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.energy.cmp(&other.energy)
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
