use std::collections::{BTreeSet, VecDeque};

fn read_file(filename: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    contents.trim().to_owned()
}

fn parse_input(input: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut player_one = VecDeque::new();
    let mut player_two = VecDeque::new();
    let mut read_one = true;

    for line in input.lines() {
        if line.chars().count() == 0 {
            read_one = false;
            continue;
        }
        if line.contains("Player") {
            continue;
        }

        if read_one {
            player_one.push_back(line.parse().unwrap());
        } else {
            player_two.push_back(line.parse().unwrap());
        }
    }
    (player_one, player_two)
}

#[derive(Eq, PartialEq)]
enum GameState {
    Over,
    Continue,
}

fn play_round(player_one: &mut VecDeque<usize>, player_two: &mut VecDeque<usize>) -> GameState {
    if player_one.is_empty() || player_two.is_empty() {
        return GameState::Over;
    }

    let n_one = player_one.pop_front().unwrap();
    let n_two = player_two.pop_front().unwrap();

    if n_one > n_two {
        player_one.push_back(n_one);
        player_one.push_back(n_two);
    } else if n_two > n_one {
        player_two.push_back(n_two);
        player_two.push_back(n_one);
    } else {
        panic!("broken game state");
    }

    if player_one.is_empty() || player_two.is_empty() {
        GameState::Over
    } else {
        GameState::Continue
    }
}

enum GameResultRecursive {
    WinOne,
    WinTwo,
}

fn play_game_recursive(
    mut player_one: VecDeque<usize>,
    mut player_two: VecDeque<usize>,
) -> (VecDeque<usize>, GameResultRecursive) {
    let mut prev_states = BTreeSet::new();
    while !player_one.is_empty() && !player_two.is_empty() {
        if prev_states.contains(&(player_one.clone(), player_two.clone())) {
            return (player_one, GameResultRecursive::WinOne);
        } else {
            prev_states.insert((player_one.clone(), player_two.clone()));
        }

        let n_one = player_one.pop_front().unwrap();
        let n_two = player_two.pop_front().unwrap();

        let result = if player_one.len() >= n_one && player_two.len() >= n_two {
            let subdeck_one = player_one.iter().take(n_one).cloned().collect();
            let subdeck_two = player_two.iter().take(n_two).cloned().collect();
            let tmp = play_game_recursive(subdeck_one, subdeck_two);
            tmp.1
        } else if n_one > n_two {
            GameResultRecursive::WinOne
        } else if n_two > n_one {
            GameResultRecursive::WinTwo
        } else {
            panic!("broken game state");
        };

        match result {
            GameResultRecursive::WinOne => {
                player_one.push_back(n_one);
                player_one.push_back(n_two);
            }
            GameResultRecursive::WinTwo => {
                player_two.push_back(n_two);
                player_two.push_back(n_one);
            }
        }
    }

    if player_one.is_empty() {
        (player_two, GameResultRecursive::WinTwo)
    } else {
        (player_one, GameResultRecursive::WinOne)
    }
}

fn compute_solution_part_one(input: &str) -> usize {
    let (mut player_one, mut player_two) = parse_input(input);
    let mut game_state = GameState::Continue;

    while game_state == GameState::Continue {
        game_state = play_round(&mut player_one, &mut player_two);
    }

    let player_win = if !player_one.is_empty() {
        player_one
    } else {
        player_two
    };

    player_win
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
        .sum()
}

fn compute_solution_part_two(input: &str) -> usize {
    let (player_one, player_two) = parse_input(input);
    let (player_win, _) = play_game_recursive(player_one, player_two);
    player_win
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
        .sum()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d22 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
