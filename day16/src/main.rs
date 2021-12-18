use std::{env, fmt::Write, fs};

enum Packet {
    Literal(LiteralPacket),
    Operation(OperationPacket),
}

struct LiteralPacket {
    value: usize,
    version: u8,
}

struct OperationPacket {
    children: Vec<Packet>,
    packet_type: u8,
    version: u8,
}

impl Packet {
    fn evaluate(&self) -> usize {
        match self {
            Packet::Literal(lp) => lp.value,
            Packet::Operation(op) => {
                let mut child_values = op.children.iter().map(Packet::evaluate);
                match op.packet_type {
                    0 => child_values.sum(),
                    1 => child_values.reduce(|acc, n| acc * n).unwrap(),
                    2 => child_values.min().unwrap(),
                    3 => child_values.max().unwrap(),
                    5 => {
                        if child_values.next().unwrap() > child_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        if child_values.next().unwrap() < child_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        if child_values.next().unwrap() == child_values.next().unwrap() {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Invalid type found"),
                }
            }
        }
    }
}

struct StringReader<'a>(&'a str);

impl<'a> StringReader<'a> {
    fn read(&mut self, bytes: usize) -> &'a str {
        let (result, next) = self.0.split_at(bytes);
        self.0 = next;
        result
    }
}

fn parse_packets(bits: &str) -> Vec<Packet> {
    let mut reader = StringReader(bits);
    let mut packets: Vec<Packet> = Vec::new();
    let mut parents: Vec<(OperationPacket, u16)> = Vec::new();

    loop {
        let vstring = reader.read(3);
        let version: u8 = u8::from_str_radix(vstring, 2).unwrap();
        // println!("Version string: {}", vstring);
        // println!("Version: {}", version);

        let packet_type: u8 = u8::from_str_radix(reader.read(3), 2).unwrap();
        // println!("Type: {}", packet_type);

        let mut maybe_packet = if packet_type == 4 {
            let mut literal_bits = String::new();
            loop {
                let last = reader.read(1) == "0";
                literal_bits.write_str(reader.read(4)).unwrap();
                if last {
                    break;
                }
            }
            Some(Packet::Literal(LiteralPacket {
                value: usize::from_str_radix(&literal_bits[..], 2).unwrap(),
                version,
            }))
        } else {
            let length_type = reader.read(1);

            // println!("Length type: {}", length_type);

            if length_type == "0" {
                let sub_length: usize = usize::from_str_radix(reader.read(15), 2).unwrap();

                // println!("Sub length: {}", sub_length);

                let children = parse_packets(reader.read(sub_length));
                Some(Packet::Operation(OperationPacket {
                    children,
                    packet_type,
                    version,
                }))
            } else {
                let sub_count: u16 = u16::from_str_radix(reader.read(11), 2).unwrap();
                parents.push((
                    OperationPacket {
                        children: Vec::new(),
                        packet_type,
                        version,
                    },
                    sub_count,
                ));
                None
            }
        };

        loop {
            let mut should_pop = false;

            if let Some(packet) = maybe_packet {
                if let Some(parent) = parents.last_mut() {
                    parent.0.children.push(packet);
                    parent.1 -= 1;
                    should_pop = parent.1 == 0
                } else {
                    packets.push(packet);
                }
            }

            if should_pop {
                maybe_packet = parents.pop().map(|p| Packet::Operation(p.0));
            } else {
                break;
            }
        }

        if reader.0.chars().all(|c| c == '0') {
            break;
        }
    }

    packets
}

fn hex_to_bin(hex: &str) -> String {
    let mut bin = String::new();
    for c in hex.chars() {
        if c != '\n' {
            write!(bin, "{:04b}", c.to_digit(16).unwrap()).unwrap();
        }
    }
    bin
}

fn part1(packet: &Packet) -> usize {
    match packet {
        Packet::Operation(op) => {
            op.version as usize + op.children.iter().map(|p| part1(p)).sum::<usize>()
        }
        Packet::Literal(lp) => lp.version as usize,
    }
}

fn part2(packet: &Packet) -> usize {
    packet.evaluate()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Error opening file");

    let bin_str = hex_to_bin(&contents);
    // println!("{}", bin_str);
    let packet = &parse_packets(&bin_str)[0];

    println!("Part 1: {}", part1(packet));
    println!("Part 2: {}", part2(packet));
}
