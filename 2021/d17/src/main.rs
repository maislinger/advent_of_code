fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d17 <input filename>");
    } else {
        let input = read_file(&args[1]);
        run(&input);
    }
}

fn run(input: &str) {
    let target_area = Area::from_str(input);
    let vxs = compute_vxs(&target_area);
    let vys = compute_vys(&target_area);

    let mut count = 0;
    let mut max_height = 0;

    for &vx in vxs.iter() {
        for &vy in vys.iter() {
            let mut p = Probe::on_start(vx, vy);

            let mut reached_target = p.in_target_area(&target_area);
            let mut max_height_probe = p.y;

            while p.can_still_reach_target_area(&target_area) {
                p.step();
                if p.y > max_height_probe {
                    max_height_probe = p.y;
                }
                if p.in_target_area(&target_area) {
                    reached_target = true;
                }
            }

            if reached_target {
                count += 1;
                if max_height_probe > max_height {
                    max_height = max_height_probe;
                }
            }
        }
    }

    println!("Solution 1 = {}", max_height);
    println!("Solution 2 = {}", count);
}

fn compute_vxs(t: &Area) -> Vec<i64> {
    let mut result = Vec::new();

    let mut lower_bound = t.x0;
    let mut upper_bound = t.x1;

    if lower_bound < 0 && upper_bound < 0 {
        upper_bound = 0;
    } else if lower_bound > 0 && upper_bound > 0 {
        lower_bound = 0;
    }

    for vx in lower_bound..=upper_bound {
        let mut p = Probe::on_start(vx, 0);
        p.y = t.y0;

        let mut reached_target = false;

        while p.can_still_reach_target_area(t) && !reached_target {
            if p.in_target_area(t) {
                reached_target = true;
            }
            p.step();
            if p.in_target_area(t) {
                reached_target = true;
            }

            p.y = t.y0;

            if p.vx == 0 {
                // Final step
                break;
            }
        }

        if reached_target {
            result.push(vx);
        }
    }

    result
}

fn compute_vys(t: &Area) -> Vec<i64> {
    let mut result = Vec::new();

    let mut vy = 0;
    if t.y0 < vy {
        vy = t.y0;
    }

    loop {
        let mut p = Probe::on_start(1, vy);
        p.x = t.x0 - 1;

        let mut reached_target = false;
        while p.can_still_reach_target_area(t) && !reached_target {
            p.step();
            if p.in_target_area(t) {
                reached_target = true;
            }
            p.x = t.x0 - 1;
            p.vx = 1;
        }

        if reached_target {
            result.push(vy);
        }

        if !reached_target && vy > 2 * t.height() {
            break;
        }

        vy += 1;
    }

    result
}

struct Area {
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
}

impl Area {
    fn from_str(input: &str) -> Self {
        fn parse(s: &str) -> (i64, i64) {
            let mut iter = s.chars();
            iter.next();
            iter.next();

            let mut iter = iter.as_str().split("..");
            let first = iter.next().unwrap().parse().unwrap();
            let second = iter.next().unwrap().parse().unwrap();

            (first, second)
        }

        let mut input_iter = input.chars();

        for _ in 0..13 {
            input_iter.next();
        }

        let a: &str = input_iter.as_str();

        let x_str = a.split(',').next().unwrap().trim();
        let y_str = a.split(',').nth(1).unwrap().trim();

        let (mut x0, mut x1) = parse(x_str);
        let (mut y0, mut y1) = parse(y_str);

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
        }

        if y0 > y1 {
            std::mem::swap(&mut y0, &mut y1);
        }

        Self { x0, x1, y0, y1 }
    }

    fn height(&self) -> i64 {
        self.y1 - self.y0
    }
}

struct Probe {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64,
}

impl Probe {
    fn on_start(vx: i64, vy: i64) -> Self {
        Self { x: 0, y: 0, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;

        if self.vx > 0 {
            self.vx -= 1;
        } else if self.vx < 0 {
            self.vx += 1;
        }

        self.vy -= 1;
    }

    fn in_target_area(&self, t: &Area) -> bool {
        t.x0 <= self.x && self.x <= t.x1 && t.y0 <= self.y && self.y <= t.y1
    }

    fn can_still_reach_target_area(&self, t: &Area) -> bool {
        if self.vx >= 0 && self.x > t.x1 {
            false
        } else if self.vx <= 0 && self.x < t.x0 {
            false
        } else if self.vy <= 0 && self.y < t.y0 {
            false
        } else {
            true
        }
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
