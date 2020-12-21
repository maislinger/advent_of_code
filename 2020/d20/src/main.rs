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

#[derive(Debug, Copy, Clone)]
enum Orientation {
    Original,                // original image
    Clockwise,               // image rotated clockwise
    CounterClockwise,        // image rotate counter clockwise
    TurnedAround,            // image rotaed twice
    OriginalFlipped,         // original flipped along the original vertical axis
    ClockwiseFlipped,        // image rotated clockwise and flipped
    CounterClockwiseFlipped, // image rotate counter clockwise and flipped
    TurnedAroundFlipped,     // image rotaed twice and flipped
}

impl Orientation {
    fn iter() -> impl Iterator<Item = Orientation> {
        [
            Self::Original,
            Self::Clockwise,
            Self::CounterClockwise,
            Self::TurnedAround,
            Self::OriginalFlipped,
            Self::ClockwiseFlipped,
            Self::CounterClockwiseFlipped,
            Self::TurnedAroundFlipped,
        ]
        .iter()
        .copied()
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Top,
    Down,
}

#[derive(Debug)]
struct Tile {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Tile {
    fn new(width: usize, height: usize, data: Vec<bool>) -> Self {
        assert_eq!(width * height, data.len());
        Self {
            width,
            height,
            data,
        }
    }

    fn read_pixel_unrotated(&self, i: usize, j: usize) -> bool {
        let index = i * self.width + j;
        self.data[index]
    }

    fn read_pixel(&self, i: usize, j: usize, orientation: Orientation) -> bool {
        match orientation {
            Orientation::Original => self.read_pixel_unrotated(i, j),
            Orientation::Clockwise => self.read_pixel_unrotated(self.height - 1 - j, i),
            Orientation::CounterClockwise => self.read_pixel_unrotated(j, self.width - 1 - i),
            Orientation::TurnedAround => {
                self.read_pixel_unrotated(self.height - 1 - i, self.width - 1 - j)
            }
            Orientation::OriginalFlipped => self.read_pixel_unrotated(i, self.width - 1 - j),
            Orientation::ClockwiseFlipped => {
                self.read_pixel_unrotated(self.height - 1 - j, self.width - 1 - i)
            }
            Orientation::CounterClockwiseFlipped => self.read_pixel_unrotated(j, i),
            Orientation::TurnedAroundFlipped => self.read_pixel_unrotated(self.height - 1 - i, j),
        }
    }

    fn write_pixel_unrotated(&mut self, i: usize, j: usize, newval: bool) {
        let index = i * self.width + j;
        self.data[index] = newval;
    }

    fn write_pixel(&mut self, i: usize, j: usize, orientation: Orientation, newval: bool) {
        match orientation {
            Orientation::Original => self.write_pixel_unrotated(i, j, newval),
            Orientation::Clockwise => self.write_pixel_unrotated(self.height - 1 - j, i, newval),
            Orientation::CounterClockwise => {
                self.write_pixel_unrotated(j, self.width - 1 - i, newval)
            }
            Orientation::TurnedAround => {
                self.write_pixel_unrotated(self.height - 1 - i, self.width - 1 - j, newval)
            }
            Orientation::OriginalFlipped => {
                self.write_pixel_unrotated(i, self.width - 1 - j, newval)
            }
            Orientation::ClockwiseFlipped => {
                self.write_pixel_unrotated(self.height - 1 - j, self.width - 1 - i, newval)
            }
            Orientation::CounterClockwiseFlipped => self.write_pixel_unrotated(j, i, newval),
            Orientation::TurnedAroundFlipped => {
                self.write_pixel_unrotated(self.height - 1 - i, j, newval)
            }
        }
    }

    fn dims_of_orientation(&self, orientation: Orientation) -> (usize, usize) {
        match orientation {
            Orientation::Original
            | Orientation::TurnedAround
            | Orientation::OriginalFlipped
            | Orientation::TurnedAroundFlipped => (self.width, self.height),
            Orientation::Clockwise
            | Orientation::CounterClockwise
            | Orientation::ClockwiseFlipped
            | Orientation::CounterClockwiseFlipped => (self.height, self.width),
        }
    }

    fn have_matching_borders(
        &self,
        self_orientation: Orientation,
        other: &Self,
        other_orientation: Orientation,
        from_direction: Direction,
    ) -> bool {
        assert_eq!(self.width, other.width);
        assert_eq!(self.height, other.height);
        let (mut i_self, mut j_self, mut i_other, mut j_other) = match from_direction {
            Direction::Left => (0, 0, 0, self.width - 1),
            Direction::Right => (0, self.width - 1, 0, 0),
            Direction::Top => (0, 0, self.height - 1, 0),
            Direction::Down => (self.height - 1, 0, 0, 0),
        };

        let n = match from_direction {
            Direction::Left | Direction::Right => self.height,
            Direction::Top | Direction::Down => self.width,
        };

        for _ in 0..n {
            if self.read_pixel(i_self, j_self, self_orientation)
                != other.read_pixel(i_other, j_other, other_orientation)
            {
                return false;
            }
            match from_direction {
                Direction::Left | Direction::Right => {
                    i_self += 1;
                    i_other += 1;
                }
                Direction::Top | Direction::Down => {
                    j_self += 1;
                    j_other += 1;
                }
            }
        }

        true
    }

    fn find_orientation(
        &self,
        self_orientation: Orientation,
        other: &Self,
        from_direction: Direction,
    ) -> Option<Orientation> {
        for other_orientation in Orientation::iter() {
            if self.have_matching_borders(
                self_orientation,
                other,
                other_orientation,
                from_direction,
            ) {
                return Some(other_orientation);
            }
        }
        None
    }
}

fn parse_input(input: &str) -> BTreeMap<usize, Tile> {
    let mut result = BTreeMap::new();

    let mut id = None;
    let mut n = None;
    let mut data = Vec::new();

    for line in input.lines() {
        if line.contains("Tile") {
            id = Some(
                line.split_whitespace()
                    .nth(1)
                    .unwrap()
                    .split(':')
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap(),
            );
            data = Vec::new();
            continue;
        }
        if line.chars().count() == 0 {
            result.insert(id.unwrap(), Tile::new(n.unwrap(), n.unwrap(), data.clone()));
            id = None;
            n = None;
            continue;
        }
        n = Some(line.chars().count());
        for c in line.chars() {
            match c {
                '#' => data.push(true),
                '.' => data.push(false),
                _ => panic!("unknown image data"),
            }
        }
    }

    if n.is_some() {
        result.insert(id.unwrap(), Tile::new(n.unwrap(), n.unwrap(), data));
    }

    let n = result.iter().next().unwrap().1.width;

    for (_, tile) in result.iter() {
        assert_eq!(n, tile.width);
    }

    result
}

fn relative_direction(parent: (i64, i64), child: (i64, i64)) -> Direction {
    // child is ... of Parent
    let x = parent.0;
    let y = parent.1;
    let xc = child.0;
    let yc = child.1;
    if x < xc && y == yc {
        Direction::Right
    } else if x > xc && y == yc {
        Direction::Left
    } else if x == xc && y < yc {
        Direction::Top
    } else if x == xc && y > yc {
        Direction::Down
    } else {
        panic!("child is ontop of parent");
    }
}

fn find_all_tiles_orientation(
    tiles: &BTreeMap<usize, Tile>,
) -> BTreeMap<(i64, i64), (usize, Orientation)> {
    let origin_tile = *tiles.iter().map(|(id, _)| id).min().unwrap();

    // coordinate --> (id, Orientiation)
    let mut assigned: BTreeMap<(i64, i64), (usize, Orientation)> = BTreeMap::new();
    assigned.insert((0, 0), (origin_tile, Orientation::Original));

    // coordinate --> where to match
    let mut coordinates_todo: BTreeMap<(i64, i64), (i64, i64)> = BTreeMap::new();
    coordinates_todo.insert((1, 0), (0, 0));
    coordinates_todo.insert((-1, 0), (0, 0));
    coordinates_todo.insert((0, 1), (0, 0));
    coordinates_todo.insert((0, -1), (0, 0));

    let mut ids_todo: BTreeSet<usize> = tiles.iter().map(|(id, _)| id).cloned().collect();
    ids_todo.remove(&origin_tile);

    while !ids_todo.is_empty() {
        'outer: for (&(x, y), &(x0, y0)) in coordinates_todo.iter() {
            let from_direction = relative_direction((x0, y0), (x, y));
            let (parent_id, parent_orientiation) = assigned[&(x0, y0)];
            let parent = &tiles[&parent_id];
            for &candidate_id in ids_todo.iter() {
                let candidate = &tiles[&candidate_id];
                if let Some(new_orientation) =
                    parent.find_orientation(parent_orientiation, candidate, from_direction)
                {
                    assigned.insert((x, y), (candidate_id, new_orientation));
                    for &(xn, yn) in [(x + 1, y), (x - 1, y), (x, y - 1), (x, y + 1)].iter() {
                        if !assigned.contains_key(&(xn, yn)) {
                            coordinates_todo.insert((xn, yn), (x, y));
                        }
                    }
                    ids_todo.remove(&candidate_id);
                    break 'outer;
                }
            }
        }
    }

    assigned
}

fn compute_solution_part_one(input: &str) -> usize {
    let tiles = parse_input(input);
    let assigned = find_all_tiles_orientation(&tiles);

    let max_x = *assigned.keys().map(|(x, _)| x).max().unwrap();
    let min_x = *assigned.keys().map(|(x, _)| x).min().unwrap();
    let max_y = *assigned.keys().map(|(_, y)| y).max().unwrap();
    let min_y = *assigned.keys().map(|(_, y)| y).min().unwrap();

    let mut result = 1;
    for &(x, y) in [
        (min_x, min_y),
        (min_x, max_y),
        (max_x, min_y),
        (max_x, max_y),
    ]
    .iter()
    {
        let (id, _) = assigned[&(x, y)];
        result *= id;
    }
    result
}

fn merge_mini_tiles(
    tiles: &BTreeMap<usize, Tile>,
    assigned: &BTreeMap<(i64, i64), (usize, Orientation)>,
) -> Tile {
    let n = tiles.iter().map(|(_, t)| t.width).next().unwrap();

    let max_x = *assigned.keys().map(|(x, _)| x).max().unwrap();
    let min_x = *assigned.keys().map(|(x, _)| x).min().unwrap();
    let max_y = *assigned.keys().map(|(_, y)| y).max().unwrap();
    let min_y = *assigned.keys().map(|(_, y)| y).min().unwrap();

    let width_in_tiles = (max_x - min_x + 1) as usize;
    let height_in_tiles = (max_y - min_y + 1) as usize;

    let width = width_in_tiles * (n - 2);
    let height = height_in_tiles * (n - 2);

    let mut data = Vec::with_capacity(width * height);

    let mut i_mini_tile = 0;
    let mut j_mini_tile = 0;
    let mut x_tile = min_x;
    let mut y_tile = max_y;
    let (mut mini_tile_id, mut mini_tile_orientation) = assigned[&(x_tile, y_tile)];
    let mut mini_tile = &tiles[&mini_tile_id];

    loop {
        if j_mini_tile == 0 {
            let tmp = assigned[&(x_tile, y_tile)];
            mini_tile_id = tmp.0;
            mini_tile_orientation = tmp.1;
            mini_tile = &tiles[&mini_tile_id];
        } else if j_mini_tile == n - 1 {
            j_mini_tile = 0;
            x_tile += 1;
            if x_tile > max_x {
                x_tile = min_x;
                i_mini_tile += 1;
            }
            if i_mini_tile >= n - 1 {
                i_mini_tile = 0;
                y_tile -= 1;
            }
            if y_tile < min_y {
                break;
            }
            continue;
        }

        if j_mini_tile != 0 && j_mini_tile != n - 1 && i_mini_tile != 0 && i_mini_tile != n - 1 {
            let value = mini_tile.read_pixel(i_mini_tile, j_mini_tile, mini_tile_orientation);
            data.push(value);
        }

        j_mini_tile += 1
    }
    Tile::new(width, height, data)
}

fn check_for_monster(i: usize, j: usize, tile: &Tile, orientation: Orientation) -> bool {
    let i = i as i64;
    let j = j as i64;
    let deltas: [(i64, i64); 15] = [
        (0, 0),
        (1, -18),
        (1, -13),
        (1, -12),
        (1, -7),
        (1, -6),
        (1, -1),
        (1, 0),
        (1, 1),
        (2, -17),
        (2, -14),
        (2, -11),
        (2, -8),
        (2, -5),
        (2, -2),
    ];

    let (width, height) = tile.dims_of_orientation(orientation);
    for (delta_i, delta_j) in deltas.iter() {
        let i0 = i + delta_i;
        let j0 = j + delta_j;
        if i0 < 0 || j0 < 0 {
            return false;
        }
        let i0 = i0 as usize;
        let j0 = j0 as usize;
        if i0 >= height || j0 >= width {
            return false;
        }
        if !tile.read_pixel(i0, j0, orientation) {
            return false;
        }
    }

    true
}

fn delete_monster(i: usize, j: usize, tile: &mut Tile, orientation: Orientation) {
    let i = i as i64;
    let j = j as i64;
    let deltas: [(i64, i64); 15] = [
        (0, 0),
        (1, -18),
        (1, -13),
        (1, -12),
        (1, -7),
        (1, -6),
        (1, -1),
        (1, 0),
        (1, 1),
        (2, -17),
        (2, -14),
        (2, -11),
        (2, -8),
        (2, -5),
        (2, -2),
    ];

    let (width, height) = tile.dims_of_orientation(orientation);
    for (delta_i, delta_j) in deltas.iter() {
        let i0 = i + delta_i;
        let j0 = j + delta_j;
        if i0 < 0 || j0 < 0 {
            panic!("invalid coordinates")
        }
        let i0 = i0 as usize;
        let j0 = j0 as usize;
        if i0 >= height || j0 >= width {
            panic!("invalid coordinates")
        }
        tile.write_pixel(i0, j0, orientation, false);
    }
}

fn compute_solution_part_two(input: &str) -> usize {
    let tiles = parse_input(input);
    let assigned = find_all_tiles_orientation(&tiles);
    let mut merged = merge_mini_tiles(&tiles, &assigned);

    let mut max_orientation = Orientation::Original;
    let mut max_monster_coords = Vec::new();

    for orientation in Orientation::iter() {
        let mut count = 0;
        let mut coords = Vec::new();
        let (width, height) = merged.dims_of_orientation(orientation);
        for i in 0..height {
            for j in 0..width {
                if check_for_monster(i, j, &merged, orientation) {
                    count += 1;
                    coords.push((i, j));
                }
            }
        }
        if count > max_monster_coords.len() {
            max_monster_coords = coords;
            max_orientation = orientation;
        }
    }

    for &(i, j) in max_monster_coords.iter() {
        delete_monster(i, j, &mut merged, max_orientation);
    }

    merged.data.iter().filter(|v| **v).count()
}

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d20 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}
