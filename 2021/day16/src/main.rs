use bit_vec::BitVec;

use std::io::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PacketType {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    Gt,
    Lt,
    Eq,
}

impl From<u8> for PacketType {
    fn from(type_id: u8) -> Self {
        match type_id {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Min,
            3 => Self::Max,
            4 => Self::Literal,
            5 => Self::Gt,
            6 => Self::Lt,
            7 => Self::Eq,
            _ => panic!("Invalid type ID: {}", type_id),
        }
    }
}

#[derive(Clone, Debug)]
enum Payload {
    Literal(usize),
    Operands(Vec<Packet>),
}

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    payload: Payload,
}

impl Packet {
    pub fn version_sum(&self) -> usize {
        self.version as usize
            + match &self.payload {
                Payload::Literal(_) => 0,
                Payload::Operands(packets) => packets.iter().map(|pkt| pkt.version_sum()).sum(),
            }
    }

    pub fn eval(&self) -> usize {
        match &self.payload {
            Payload::Literal(num) => *num,
            Payload::Operands(operands) => match self.packet_type {
                PacketType::Sum => operands.iter().map(|packet| packet.eval()).sum(),
                PacketType::Product => operands.iter().map(|packet| packet.eval()).product(),
                PacketType::Min => operands.iter().map(|packet| packet.eval()).min().unwrap(),
                PacketType::Max => operands.iter().map(|packet| packet.eval()).max().unwrap(),
                PacketType::Literal => panic!("Nonliteral payload on a literal packet?"),
                PacketType::Gt => {
                    assert_eq!(operands.len(), 2);
                    if operands[0].eval() > operands[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                PacketType::Lt => {
                    assert_eq!(operands.len(), 2);
                    if operands[0].eval() < operands[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                PacketType::Eq => {
                    assert_eq!(operands.len(), 2);
                    if operands[0].eval() == operands[1].eval() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

fn decode_hex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect()
}

struct Parser {
    bits: BitVec<u32>,
    offset: usize,
}

impl Parser {
    pub fn new(bits: BitVec<u32>) -> Self {
        Self { bits, offset: 0 }
    }

    pub fn parse_packet(&mut self) -> Packet {
        let (version, packet_type) = self.parse_header();
        let payload = match packet_type {
            PacketType::Literal => Payload::Literal(self.parse_literal()),
            _ => {
                let subpackets = if self.get() {
                    let num_subpackets = self.parse_number(11);

                    let mut subpackets = vec![];
                    for _ in 0..num_subpackets {
                        subpackets.push(self.parse_packet());
                    }
                    subpackets
                } else {
                    let subpackets_size = self.parse_number(15);

                    let subpacket_start = self.offset;
                    let mut subpackets = vec![];
                    while self.offset < subpacket_start + subpackets_size {
                        subpackets.push(self.parse_packet());
                    }
                    assert_eq!(self.offset, subpacket_start + subpackets_size);
                    subpackets
                };
                Payload::Operands(subpackets)
            }
        };
        Packet {
            version,
            packet_type,
            payload,
        }
    }

    fn get(&mut self) -> bool {
        let bit = self.bits.get(self.offset).unwrap();
        self.offset += 1;
        bit
    }

    fn parse_number(&mut self, size: usize) -> usize {
        let mut num = 0;
        for _ in 0..size {
            num *= 2;
            num += if self.get() { 1 } else { 0 };
        }
        num
    }

    fn parse_header(&mut self) -> (u8, PacketType) {
        let version = self.parse_number(3) as u8;
        let type_id = PacketType::from(self.parse_number(3) as u8);
        (version, type_id)
    }

    fn parse_literal(&mut self) -> usize {
        let mut literal = 0;
        let mut keep_going = true;
        while keep_going {
            keep_going = self.get();
            literal *= 16;
            literal += self.parse_number(4);
        }
        literal
    }
}

fn main() {
    let bits = BitVec::from_bytes(
        &std::io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .map(|line| decode_hex(&line))
            .next()
            .unwrap(),
    );

    let packet = Parser::new(bits).parse_packet();
    println!(
        "{:?}. Version sum: {}. Evaluation: {}",
        packet,
        packet.version_sum(),
        packet.eval()
    );
}

#[cfg(test)]
fn hex_to_packet(hex: &str) -> Packet {
    let bits = BitVec::from_bytes(&decode_hex(hex));
    println!("Bits: {:?}", bits);
    Parser::new(bits).parse_packet()
}

#[test]
fn test_sample1() {
    let packet = hex_to_packet("38006F45291200");
    assert_eq!(packet.version, 1);
    assert_eq!(packet.packet_type, PacketType::Lt);
    match packet.payload {
        Payload::Operands(subpackets) => {
            assert_eq!(subpackets.len(), 2);
            assert_eq!(subpackets[0].packet_type, PacketType::Literal);
            match subpackets[0].payload {
                Payload::Literal(num) => assert_eq!(num, 10),
                _ => panic!("Incorrect payload type"),
            }
            assert_eq!(subpackets[1].packet_type, PacketType::Literal);
            match subpackets[1].payload {
                Payload::Literal(num) => assert_eq!(num, 20),
                _ => panic!("Incorrect payload type"),
            }
        }
        _ => panic!("Incorrect payload type"),
    }
}

#[test]
fn test_sample2() {
    let packet = hex_to_packet("EE00D40C823060");
    assert_eq!(packet.version, 7);
    assert_eq!(packet.packet_type, PacketType::Max);
    match packet.payload {
        Payload::Operands(subpackets) => {
            assert_eq!(subpackets.len(), 3);
            assert_eq!(subpackets[0].packet_type, PacketType::Literal);
            match subpackets[0].payload {
                Payload::Literal(num) => assert_eq!(num, 1),
                _ => panic!("Incorrect payload type"),
            }
            assert_eq!(subpackets[1].packet_type, PacketType::Literal);
            match subpackets[1].payload {
                Payload::Literal(num) => assert_eq!(num, 2),
                _ => panic!("Incorrect payload type"),
            }
            assert_eq!(subpackets[2].packet_type, PacketType::Literal);
            match subpackets[2].payload {
                Payload::Literal(num) => assert_eq!(num, 3),
                _ => panic!("Incorrect payload type"),
            }
        }
        _ => panic!("Incorrect payload type"),
    }
}

#[test]
fn test_sample3() {
    let packet = hex_to_packet("8A004A801A8002F478");
    assert_eq!(packet.version, 4);

    let subpacket = match packet.payload {
        Payload::Operands(subpackets) => {
            assert_eq!(subpackets.len(), 1);
            subpackets.into_iter().next().unwrap()
        }
        _ => panic!("Incorrect payload type"),
    };
    assert_eq!(subpacket.version, 1);

    let subpacket = match subpacket.payload {
        Payload::Operands(subpackets) => {
            assert_eq!(subpackets.len(), 1);
            subpackets.into_iter().next().unwrap()
        }
        _ => panic!("Incorrect payload type"),
    };
    assert_eq!(subpacket.version, 5);
    let subpacket = match subpacket.payload {
        Payload::Operands(subpackets) => {
            assert_eq!(subpackets.len(), 1);
            subpackets.into_iter().next().unwrap()
        }
        _ => panic!("Incorrect payload type"),
    };
    assert_eq!(subpacket.version, 6);
    assert_eq!(subpacket.packet_type, PacketType::Literal);
}

#[test]
fn test_sample4() {
    hex_to_packet("620080001611562C8802118E34");
}

#[test]
fn test_sample5() {
    hex_to_packet("C0015000016115A2E0802F182340");
}

#[test]
fn test_sample6() {
    hex_to_packet("A0016C880162017C3686B18A3D4780");
}
