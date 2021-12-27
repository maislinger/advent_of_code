use std::collections::BTreeMap;

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

fn compute_solution_part_one(input: &str) -> usize {
    let mut players = parse_input(input);
    let mut player_index = 0;
    let mut die = DeterministicDie::new();
    let loser_index;
    loop {
        let n_fields = die.roll_n(3);
        players[player_index].step(n_fields);

        player_index = 1 - player_index;

        if players[0].points >= 1000 {
            loser_index = 1;
            break;
        }
        if players[1].points >= 1000 {
            loser_index = 0;
            break;
        }
    }

    players[loser_index].points * die.n_rolls
}

fn compute_solution_part_two(input: &str) -> usize {
    let mut game_info: BTreeMap<[Player; 2], usize> = {
        let players = parse_input(input);
        let mut game_info = BTreeMap::new();
        game_info.insert(players, 1);
        game_info
    };

    // die sum, possibilities
    let die_count = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let mut player_index = 0;
    let mut changed = true;
    let winning_score = 21;

    let mut wins_one = 0;
    let mut wins_two = 0;

    while changed {
        let mut new_game_info = BTreeMap::new();
        changed = false;

        game_info.retain(|players, n| {
            if players[0].points >= winning_score {
                wins_one += *n;
                false
            } else if players[1].points >= winning_score {
                wins_two += *n;
                false
            } else {
                true
            }
        });

        for (players, n) in game_info.iter() {
            changed = true;
            for &(n_fields, count) in &die_count {
                let mut players = *players;
                players[player_index].step(n_fields);

                let c = new_game_info.entry(players).or_insert(0);
                *c += n * count;
            }
        }

        player_index = 1 - player_index;
        game_info = new_game_info;
    }

    if wins_one > wins_two {
        wins_one
    } else {
        wins_two
    }
}

fn parse_input(input: &str) -> [Player; 2] {
    let input = input.trim();

    let field_one = input
        .lines()
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let field_two = input
        .lines()
        .nth(1)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let player_one = Player::new(field_one, 0);
    let player_two = Player::new(field_two, 0);

    [player_one, player_two]
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Player {
    field: usize,
    points: usize,
}

impl Player {
    fn new(field: usize, points: usize) -> Self {
        Self { field, points }
    }

    fn step(&mut self, n_fields: usize) {
        self.field -= 1;
        self.field += n_fields;
        self.field = self.field % 10 + 1;
        self.points += self.field;
    }
}

struct DeterministicDie {
    next_val: usize,
    n_rolls: usize,
}

impl DeterministicDie {
    fn new() -> Self {
        Self {
            next_val: 1,
            n_rolls: 0,
        }
    }

    fn roll(&mut self) -> usize {
        self.n_rolls += 1;
        let retval = self.next_val;
        self.next_val += 1;
        if self.next_val >= 101 {
            self.next_val = 1;
        }
        retval
    }

    fn roll_n(&mut self, n: usize) -> usize {
        let mut result = 0;

        for _ in 0..n {
            result += self.roll();
        }

        result
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
