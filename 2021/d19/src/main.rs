use std::collections::{BTreeMap, BTreeSet};

fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d19 <input filename>");
    } else {
        let input = read_file(&args[1]);
        run(&input);
    }
}

fn run(input: &str) {
    let orientations = compute_orientations();
    let mut scanners = parse_input(input);

    let fingerprints: Vec<_> = scanners.iter().map(|s| s.fingerprint()).collect();

    scanners[0].position = Some(Vector3::new(0, 0, 0));
    let mut todo = vec![0];

    while !todo.is_empty() {
        if scanners.iter().all(|s| s.position.is_some()) {
            break;
        }

        let i = todo.pop().unwrap();
        let diffs_i = scanners[i].beacon_diffs_with_orientation(&orientations[0]);

        for j in 0..scanners.len() {
            if i == j {
                continue;
            }

            if scanners[j].position.is_some() {
                continue;
            }

            let fingerprint_overlap: usize = fingerprints[j]
                .iter()
                .map(|(k, v)| {
                    if let Some(v2) = fingerprints[i].get(k) {
                        if v2 < v {
                            *v2
                        } else {
                            *v
                        }
                    } else {
                        0
                    }
                })
                .sum();

            if fingerprint_overlap < 66 {
                continue;
            }

            let mut max_overlap = None;
            let mut argmax = 0;

            for (k, orientation) in orientations.iter().enumerate() {
                let mut diffs_j = scanners[j].beacon_diffs_with_orientation(orientation);
                diffs_j.retain(|k, _| diffs_i.contains_key(k));
                let overlap = diffs_j
                    .iter()
                    .map(|(k, v)| {
                        let t = diffs_i[k];
                        if t < *v {
                            t
                        } else {
                            *v
                        }
                    })
                    .sum::<usize>();

                if max_overlap.is_none() || max_overlap.unwrap() < overlap {
                    max_overlap = Some(overlap);
                    argmax = k;
                }
            }

            let max_overlap = max_overlap.unwrap() / 2; // Remove double counting, a->b & b->a
            if max_overlap >= 66 {
                // 66 = 11 * 10 * ... * 2 * 1
                scanners[j].transform_to_canonical(&orientations[argmax]);
                compute_position(j, i, &mut scanners);
                todo.push(j);
            }
        }
    }

    let mut beacons = BTreeSet::new();

    for s in &scanners {
        assert!(s.position.is_some());
        for b in &s.beacons {
            beacons.insert(s.position.unwrap() + *b);
        }
    }

    println!("Solution 1 = {}", beacons.len());

    let mut max_dist = 0;

    for i in 0..scanners.len() {
        for j in i..scanners.len() {
            let dist_vec = scanners[i].position.unwrap() - scanners[j].position.unwrap();
            let dist = dist_vec.x.abs() + dist_vec.y.abs() + dist_vec.z.abs();
            if max_dist < dist {
                max_dist = dist;
            }
        }
    }

    println!("Solution 2 = {}", max_dist);
}

fn parse_input(input: &str) -> Vec<Scanner> {
    let mut result = Vec::new();

    for group in input.trim().split("---") {
        let group = group.trim();
        if group.is_empty() {
            continue;
        }
        if group.starts_with('s') {
            continue;
        }

        result.push(Scanner::from_str(group));
    }

    result
}

fn compute_position(to_compute: usize, already_known: usize, scanners: &mut Vec<Scanner>) {
    let known_set: BTreeSet<_> = scanners[already_known].beacons.iter().cloned().collect();

    for i in 0..scanners[to_compute].beacons.len() {
        for j in 0..scanners[already_known].beacons.len() {
            let delta = scanners[to_compute].beacons[i] - scanners[already_known].beacons[j];
            let new_set: BTreeSet<_> = scanners[to_compute]
                .beacons
                .iter()
                .map(|v| *v - delta)
                .collect();
            let n_overlap = known_set.intersection(&new_set).count();

            if n_overlap >= 12 {
                scanners[to_compute].position =
                    Some(scanners[already_known].position.unwrap() - delta);
                return;
            }
        }
    }

    unreachable!();
}

fn compute_orientations() -> [[Vector3; 3]; 24] {
    let mut result_set = BTreeSet::new();
    result_set.insert([
        Vector3::unit(Axis::X),
        Vector3::unit(Axis::Y),
        Vector3::unit(Axis::Z),
    ]);

    while result_set.len() < 24 {
        let mut q = None;
        for r in result_set.iter() {
            let mut k = [
                r[0].rotated90(Axis::X),
                r[1].rotated90(Axis::X),
                r[2].rotated90(Axis::X),
            ];
            if !result_set.contains(&k) {
                q = Some(k);
                break;
            }

            k = [
                r[0].rotated90(Axis::Y),
                r[1].rotated90(Axis::Y),
                r[2].rotated90(Axis::Y),
            ];
            if !result_set.contains(&k) {
                q = Some(k);
                break;
            }

            k = [
                r[0].rotated90(Axis::Y),
                r[1].rotated90(Axis::Y),
                r[2].rotated90(Axis::Y),
            ];
            if !result_set.contains(&k) {
                q = Some(k);
                break;
            }
        }

        if let Some(k) = q {
            result_set.insert(k);
        }
    }

    let mut result = [[Vector3::zero(); 3]; 24];
    result[0] = [
        Vector3::unit(Axis::X),
        Vector3::unit(Axis::Y),
        Vector3::unit(Axis::Z),
    ];
    result_set.remove(&result[0]);
    let mut i = 1;
    for r in result_set.iter() {
        result[i] = *r;
        i += 1;
    }
    result
}

#[derive(Debug)]
struct Scanner {
    position: Option<Vector3>,
    beacons: Vec<Vector3>,
}

impl Scanner {
    fn from_str(s: &str) -> Self {
        let position = None;
        let mut beacons = Vec::new();
        for line in s.lines() {
            beacons.push(Vector3::from_str(line));
        }

        Self { position, beacons }
    }

    fn transform_to_canonical(&mut self, orientation: &[Vector3; 3]) {
        for v in self.beacons.iter_mut() {
            *v = v.transformed_to_canonical(orientation);
        }
    }

    fn beacon_diffs_with_orientation(
        &self,
        orientation: &[Vector3; 3],
    ) -> BTreeMap<Vector3, usize> {
        let mut result = BTreeMap::new();

        for (i, v) in self.beacons.iter().enumerate() {
            let v = v.transformed_to_canonical(orientation);
            for (j, k) in self.beacons.iter().enumerate() {
                if i == j {
                    continue;
                }
                let k = k.transformed_to_canonical(orientation);
                let diff = v - k;
                let count = result.entry(diff).or_insert(0);
                *count += 1;
            }
        }

        result
    }

    fn fingerprint(&self) -> BTreeMap<Vector3, usize> {
        let orientation = [
            Vector3::unit(Axis::X),
            Vector3::unit(Axis::Y),
            Vector3::unit(Axis::Z),
        ];
        let intermediate_result = self.beacon_diffs_with_orientation(&orientation);

        let mut result = BTreeMap::new();

        for (k, v) in intermediate_result.iter() {
            let mut mags = [k.x.abs(), k.y.abs(), k.z.abs()];
            mags.sort_unstable();
            let e = result
                .entry(Vector3::new(mags[0], mags[1], mags[2]))
                .or_insert(0);
            *e += *v;
        }

        for (_, v) in result.iter_mut() {
            *v /= 2;
        }

        result
    }
}

#[derive(Copy, Clone, Debug)]
enum Axis {
    X,
    Y,
    Z,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Vector3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector3 {
    fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn from_str(s: &str) -> Self {
        let mut s_iter = s.trim().split(',');
        let x = s_iter.next().unwrap().trim().parse().unwrap();
        let y = s_iter.next().unwrap().trim().parse().unwrap();
        let z = s_iter.next().unwrap().trim().parse().unwrap();
        Self::new(x, y, z)
    }

    fn rotated90(&self, axis: Axis) -> Self {
        match axis {
            Axis::X => Self::new(self.x, -self.z, self.y),
            Axis::Y => Self::new(self.z, self.y, -self.x),
            Axis::Z => Self::new(-self.y, self.x, self.z),
        }
    }

    fn transformed_to_canonical(&self, units: &[Vector3; 3]) -> Self {
        let x = self.x * units[0].x + self.y * units[0].y + self.z * units[0].z;
        let y = self.x * units[1].x + self.y * units[1].y + self.z * units[1].z;
        let z = self.x * units[2].x + self.y * units[2].y + self.z * units[2].z;
        Self::new(x, y, z)
    }

    fn unit(axis: Axis) -> Self {
        match axis {
            Axis::X => Self::new(1, 0, 0),
            Axis::Y => Self::new(0, 1, 0),
            Axis::Z => Self::new(0, 0, 1),
        }
    }
}

impl std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
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
