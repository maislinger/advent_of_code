extern crate regex;

use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
enum Pixel {
    On,
    Off,
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Pattern {
    pixels: Vec<Pixel>,
    size: usize,
}

impl Pattern {
    fn new(pixels: Vec<Pixel>) -> Pattern {
        assert!([4, 9, 16].iter().any(|&l| pixels.len() == l));
        let size = [2, 3, 4].iter().find(|&s| s * s == pixels.len()).unwrap();
        Pattern {
            pixels: pixels,
            size: *size,
        }
    }

    fn clone_from_indices(&self, indices: &[usize]) -> Pattern {
        let res_p = indices.iter().map(|&i| self.pixels[i].clone()).collect();
        Pattern::new(res_p)
    }

    fn rotate(&self) -> Pattern {
        let indices = match self.pixels.len() {
            4 => vec![1, 3, 0, 2],
            9 => vec![2, 5, 8, 1, 4, 7, 0, 3, 6],
            _ => unreachable!(),
        };
        self.clone_from_indices(&indices)
    }

    fn flip(&self) -> Pattern {
        let indices = match self.pixels.len() {
            4 => vec![1, 0, 3, 2],
            9 => vec![2, 1, 0, 5, 4, 3, 8, 7, 6],
            _ => unreachable!(),
        };
        self.clone_from_indices(&indices)
    }

    fn iter_permutations(&self) -> PatternPermutationsIter {
        PatternPermutationsIter::new(self)
    }
}

struct PatternPermutationsIter {
    pattern: Pattern,
    n: usize,
}

impl PatternPermutationsIter {
    fn new(pattern: &Pattern) -> PatternPermutationsIter {
        PatternPermutationsIter {
            pattern: pattern.clone(),
            n: 0,
        }
    }
}

impl Iterator for PatternPermutationsIter {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        if self.n == 5 {
            self.pattern = self.pattern.flip();
            Some(self.pattern.clone())
        } else if self.n < 9 {
            self.pattern = self.pattern.rotate();
            Some(self.pattern.clone())
        } else {
            None
        }
    }
}

struct Image {
    pixels: Vec<Pixel>,
    size: usize,
}

impl Image {
    fn new(pixels: Vec<Pixel>, size: usize) -> Image {
        assert_eq!(pixels.len(), size * size);
        Image {
            pixels: pixels,
            size: size,
        }
    }

    fn from_patern_iter<T>(patern_iter: T, size: usize) -> Image
    where
        T: Iterator<Item = Pattern>,
    {
        let pixels = vec![Pixel::Off; size * size];
        let mut image = Image::new(pixels, size);
        let mut i = 0;
        let mut j = 0;

        for patern in patern_iter {
            let psize = patern.size;
            image.write_pattern(&patern, i, j);
            j += psize;
            if j >= size {
                j = 0;
                i += psize;
            }
        }
        image
    }

    fn sub_indices(&self, i: usize, j: usize, psize: usize) -> Vec<usize> {
        let mut indices = Vec::new();
        let mut si = 0;
        let mut sj = 0;
        while indices.len() < psize * psize {
            let ci = i + si;
            let cj = j + sj;
            let ind = ci * self.size + cj;
            indices.push(ind);
            sj += 1;
            if sj >= psize {
                sj = 0;
                si += 1;
            }
        }
        indices
    }

    fn write_pattern(&mut self, pattern: &Pattern, i: usize, j: usize) {
        let indices = self.sub_indices(i, j, pattern.size);
        for (ind, val) in indices.iter().zip(pattern.pixels.iter()) {
            self.pixels[*ind] = val.clone();
        }
    }

    fn pattern_from_indices(&self, indices: &[usize]) -> Pattern {
        let res_p = indices.iter().map(|&i| self.pixels[i].clone()).collect();
        Pattern::new(res_p)
    }

    fn patern_iter(&self) -> SubImageIterator {
        SubImageIterator::new(self)
    }

    fn increase_size(&mut self, rules: &BTreeMap<Pattern, Pattern>) {
        let newsize = if self.size % 2 == 0 {
            let npats = self.size / 2;
            npats * 3
        } else {
            let npats = self.size / 3;
            npats * 4
        };
        let image = {
            let iter = self.patern_iter().map(|p| rules.get(&p).unwrap().clone());
            Image::from_patern_iter(iter, newsize)
        };
        *self = image;
    }
}

struct SubImageIterator<'a> {
    image: &'a Image,
    i: usize,
    j: usize,
    patternsize: usize,
}

impl<'a> SubImageIterator<'a> {
    fn new(image: &Image) -> SubImageIterator {
        let patternsize = if image.size % 2 == 0 { 2 } else { 3 };
        SubImageIterator {
            image: image,
            i: 0,
            j: 0,
            patternsize: patternsize,
        }
    }

    fn increase_coords(&mut self) {
        self.j += self.patternsize;
        if self.j >= self.image.size {
            self.j = 0;
            self.i += self.patternsize;
        }
    }

    fn pattern(&self) -> Pattern {
        let indices = self.image.sub_indices(self.i, self.j, self.patternsize);
        self.image.pattern_from_indices(&indices)
    }
}

impl<'a> Iterator for SubImageIterator<'a> {
    type Item = Pattern;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.image.size {
            let pattern = self.pattern();
            self.increase_coords();
            Some(pattern)
        } else {
            None
        }
    }
}

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

fn convert_input(input: &str) -> BTreeMap<Pattern, Pattern> {
    use regex::Regex;

    let str2 = r"(\.|#)(\.|#)".to_string();
    let str3 = r"(\.|#)(\.|#)(\.|#)".to_string();
    let str4 = r"(\.|#)(\.|#)(\.|#)(\.|#)".to_string();

    let pat2 = str2.clone() + "/" + &str2;
    let pat3 = str3.clone() + "/" + &str3 + "/" + &str3;
    let pat4 = str4.clone() + "/" + &str4 + "/" + &str4 + "/" + &str4;

    let restr2 = pat2 + " => " + &pat3;
    let restr3 = pat3 + " => " + &pat4;

    let re2: Regex = Regex::new(&restr2).unwrap();
    let re3: Regex = Regex::new(&restr3).unwrap();

    let mut result = BTreeMap::new();

    macro_rules! insert {
        ( $cap:expr, $i:expr, $j:expr ) => {
            {
                $cap.iter()
                    .skip($i)
                    .take($j)
                    .map(|c| c.unwrap().as_str())
                    .map(|s| match s {
                        "." => Pixel::Off,
                        "#" => Pixel::On,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<Pixel>>()
            }
        };
    }

    for cap in re2.captures_iter(input) {
        let inpxl = insert!(cap, 1, 4);
        let outpxl = insert!(cap, 5, 9);
        let inptn = Pattern::new(inpxl);
        let outptn = Pattern::new(outpxl);
        for p in inptn.iter_permutations() {
            result.insert(p, outptn.clone());
        }
    }
    for cap in re3.captures_iter(input) {
        let inpxl = insert!(cap, 1, 9);
        let outpxl = insert!(cap, 10, 16);
        let inptn = Pattern::new(inpxl);
        let outptn = Pattern::new(outpxl);
        for p in inptn.iter_permutations() {
            result.insert(p, outptn.clone());
        }
    }

    result
}

fn count_ones(input: &str, iterations: usize) -> usize {
    let rules = convert_input(input);
    let mut image = {
        let mut init_pixels = vec![Pixel::Off; 9];
        [1, 5, 6, 7, 8]
            .iter()
            .map(|&i| init_pixels[i] = Pixel::On)
            .count();
        Image::new(init_pixels, 3)
    };

    for _ in 0..iterations {
        image.increase_size(&rules);
    }

    image
        .pixels
        .iter()
        .filter(|&p| match *p {
            Pixel::On => true,
            Pixel::Off => false,
        })
        .count()
}

fn compute_solution_part_one(input: &str) -> usize {
    count_ones(input, 5)
}

fn compute_solution_part_two(input: &str) -> usize {
    count_ones(input, 18)
}

#[cfg(test)]
mod tests {
    #[test]
    fn examples() {
        use count_ones;

        let input = "../.# => ##./#../...
                     .#./..#/### => #..#/..../..../#..#";
        let solution = count_ones(input, 2);
        assert_eq!(solution, 12);
    }
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
