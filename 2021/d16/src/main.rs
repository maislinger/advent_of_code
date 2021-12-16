fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: d16 <input filename>");
    } else {
        let input = read_file(&args[1]);
        println!("solution 1 = {}", compute_solution_part_one(&input));
        println!("solution 2 = {}", compute_solution_part_two(&input));
    }
}

fn compute_solution_part_one(input: &str) -> u64 {
    let mut bits_stream = Vec::new();
    for c in input.chars() {
        push_bits(&mut bits_stream, c);
    }

    let (_, mut packet) = parse_packets(&bits_stream, Some(1));
    let packet = packet.pop().unwrap();

    packet.version_sum()
}

fn compute_solution_part_two(input: &str) -> u64 {
    let mut bits_stream = Vec::new();
    for c in input.chars() {
        push_bits(&mut bits_stream, c);
    }

    let (_, mut packet) = parse_packets(&bits_stream, Some(1));
    let packet = packet.pop().unwrap();

    packet.eval()
}

fn parse_packets(bits_stream: &[bool], max_packets: Option<usize>) -> (usize, Vec<Packet>) {
    let mut result = Vec::new();
    let mut index = 0;

    while index < bits_stream.len() {
        if let Some(m) = max_packets {
            if result.len() >= m {
                break;
            }
        }

        let packet_version = parse_slice(&bits_stream[index..(index + 3)]);
        index += 3;

        let packet_type_id = parse_slice(&bits_stream[index..(index + 3)]);
        index += 3;

        if packet_type_id == 4 {
            let (i, n) = parse_literal(&bits_stream[index..]);
            index += i;

            let s = PacketLiteral {
                _version: packet_version,
                val: n,
            };

            result.push(Packet::Literal(s));

            continue;
        }

        let length_type = bits_stream[index];
        index += 1;

        let mut subpackets = if length_type {
            let n_packets = parse_slice(&bits_stream[index..(index + 11)]) as usize;
            index += 11;
            let (i, packets) = parse_packets(&bits_stream[index..], Some(n_packets));
            index += i;
            packets
        } else {
            let n_bits = parse_slice(&bits_stream[index..(index + 15)]) as usize;
            index += 15;
            let (i, packets) = parse_packets(&bits_stream[index..(index + n_bits)], None);
            index += i;
            packets
        };

        let new_packet = match packet_type_id {
            0 | 1 | 2 | 3 => {
                let t = match packet_type_id {
                    0 => PacketArrayType::Sum,
                    1 => PacketArrayType::Product,
                    2 => PacketArrayType::Minimum,
                    _ => PacketArrayType::Maximum,
                };

                let s = PacketArraylike {
                    _version: packet_version,
                    t,
                    children: subpackets,
                };

                Packet::Arraylike(s)
            }
            5 | 6 | 7 | 8 => {
                assert_eq!(subpackets.len(), 2);

                let t = match packet_type_id {
                    5 => PacketBinaryType::Greater,
                    6 => PacketBinaryType::Less,
                    _ => PacketBinaryType::Equal,
                };

                let right = subpackets.pop().unwrap();
                let left = subpackets.pop().unwrap();

                let s = PacketBinary {
                    _version: packet_version,
                    t,
                    left: Box::new(left),
                    right: Box::new(right),
                };

                Packet::Binary(s)
            }
            _ => panic!("invalid packet type ID"),
        };

        result.push(new_packet);
    }

    (index, result)
}

fn parse_literal(bits_stream: &[bool]) -> (usize, u64) {
    let mut result = 0;
    let mut keep_reading = true;
    let mut index = 0;

    while keep_reading {
        keep_reading = bits_stream[index];
        result <<= 4;
        result += parse_slice(&bits_stream[(index + 1)..(index + 5)]);
        index += 5;
    }

    (index, result)
}

trait PacketTrait {
    fn version(&self) -> u64;
    fn version_sum(&self) -> u64;
    fn eval(&self) -> u64;
}

enum Packet {
    Literal(PacketLiteral),
    Arraylike(PacketArraylike),
    Binary(PacketBinary),
}

impl PacketTrait for Packet {
    fn version(&self) -> u64 {
        match self {
            Packet::Literal(p) => p.version(),
            Packet::Arraylike(p) => p.version(),
            Packet::Binary(p) => p.version(),
        }
    }

    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal(p) => p.version_sum(),
            Packet::Arraylike(p) => p.version_sum(),
            Packet::Binary(p) => p.version_sum(),
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Packet::Literal(p) => p.eval(),
            Packet::Arraylike(p) => p.eval(),
            Packet::Binary(p) => p.eval(),
        }
    }
}

struct PacketLiteral {
    _version: u64,
    val: u64,
}

impl PacketTrait for PacketLiteral {
    fn version(&self) -> u64 {
        self._version
    }

    fn version_sum(&self) -> u64 {
        self._version
    }

    fn eval(&self) -> u64 {
        self.val
    }
}

enum PacketArrayType {
    Sum,
    Product,
    Minimum,
    Maximum,
}

struct PacketArraylike {
    _version: u64,
    t: PacketArrayType,
    children: Vec<Packet>,
}

impl PacketTrait for PacketArraylike {
    fn version(&self) -> u64 {
        self._version
    }

    fn version_sum(&self) -> u64 {
        self._version + self.children.iter().map(|p| p.version_sum()).sum::<u64>()
    }

    fn eval(&self) -> u64 {
        match self.t {
            PacketArrayType::Sum => self.children.iter().map(|p| p.eval()).sum(),
            PacketArrayType::Product => self.children.iter().map(|p| p.eval()).product(),
            PacketArrayType::Minimum => self.children.iter().map(|p| p.eval()).min().unwrap(),
            PacketArrayType::Maximum => self.children.iter().map(|p| p.eval()).max().unwrap(),
        }
    }
}

enum PacketBinaryType {
    Greater,
    Less,
    Equal,
}

struct PacketBinary {
    _version: u64,
    t: PacketBinaryType,
    left: Box<Packet>,
    right: Box<Packet>,
}

impl PacketTrait for PacketBinary {
    fn version(&self) -> u64 {
        self._version
    }

    fn version_sum(&self) -> u64 {
        self._version + self.left.version_sum() + self.right.version_sum()
    }

    fn eval(&self) -> u64 {
        let left_val = self.left.eval();
        let right_val = self.right.eval();

        let ret_one = match self.t {
            PacketBinaryType::Greater => left_val > right_val,
            PacketBinaryType::Less => left_val < right_val,
            PacketBinaryType::Equal => left_val == right_val,
        };

        if ret_one {
            1
        } else {
            0
        }
    }
}

fn push_bits(bits_stream: &mut Vec<bool>, c: char) {
    let bits = match c {
        '0' => [false, false, false, false],
        '1' => [false, false, false, true],
        '2' => [false, false, true, false],
        '3' => [false, false, true, true],
        '4' => [false, true, false, false],
        '5' => [false, true, false, true],
        '6' => [false, true, true, false],
        '7' => [false, true, true, true],
        '8' => [true, false, false, false],
        '9' => [true, false, false, true],
        'A' | 'a' => [true, false, true, false],
        'B' | 'b' => [true, false, true, true],
        'C' | 'c' => [true, true, false, false],
        'D' | 'd' => [true, true, false, true],
        'E' | 'e' => [true, true, true, false],
        'F' | 'f' => [true, true, true, true],
        _ => panic!("invalid input"),
    };

    bits_stream.extend_from_slice(&bits);
}

fn parse_slice(bits: &[bool]) -> u64 {
    let mut result = 0;
    for b in bits {
        result <<= 1;
        if *b {
            result += 1;
        }
    }
    result
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
