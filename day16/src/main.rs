#![feature(test)]
#![feature(destructuring_assignment)]

extern crate test;

use nom::bits;
use ux::*;

use parse::BitParsable;

use crate::parse::{parse_bool, remaining, BitInput, BitResult};

mod parse;

#[derive(Debug, Default)]
struct Packet {
    id: u3,
    version: u3,
    literal: u64,
    subpackets: Vec<Packet>,
}

fn main() {
    let (part1_answer, part2_answer) = run(include_str!("../input"));
    println!("part 1 answer: {}", part1_answer);
    println!("part 2 answer: {}", part2_answer);
}

fn run(input: &'static str) -> (u16, u64) {
    let p = Packet::parse_hex(input);
    let part1_answer = p.sum_versions();
    let part2_answer = p.value();
    (part1_answer, part2_answer)
}

impl Packet {
    pub fn parse_hex(input: &str) -> Packet {
        let i = hex::decode(input.trim_end()).unwrap();
        Packet::parse(i)
    }

    pub fn parse(i: Vec<u8>) -> Packet {
        let (_, packet) = bits(|i| {
            let (i, packet) = Packet::parse_bits(i)?;
            Ok((i, packet))
        })(&i)
        .unwrap();
        packet
    }

    fn parse_bits(i: BitInput) -> BitResult<Packet> {
        let mut i = i;
        let mut packet: Packet = Default::default();
        (i, packet.version) = u3::parse(i)?;
        (i, packet.id) = u3::parse(i)?;

        if packet.id == u3::new(4) {
            let mut more = true;
            let mut literal = 0;
            while more {
                (i, more) = parse_bool(i)?;
                let l;
                (i, l) = u4::parse(i)?;
                let l = u64::from(l);
                literal = (literal << 4) + l;
            }
            packet.literal = literal;
        } else {
            let length_type_id;
            (i, length_type_id) = parse_bool(i)?;

            if length_type_id {
                let num_subpackets;
                (i, num_subpackets) = u11::parse(i)?;
                let num_subpackets = u16::from(num_subpackets);

                for _ in 0..num_subpackets {
                    let p;
                    (i, p) = Packet::parse_bits(i)?;
                    packet.subpackets.push(p);
                }
            } else {
                let len_subpackets;
                (i, len_subpackets) = u15::parse(i)?;
                let len_subpackets = u16::from(len_subpackets) as usize;
                let end = remaining(i) - len_subpackets;

                while remaining(i) > end {
                    let p;
                    (i, p) = Packet::parse_bits(i)?;
                    packet.subpackets.push(p);
                }
            }
        }

        Ok((i, packet))
    }

    fn sum_versions(&self) -> u16 {
        u16::from(self.version)
            + self
                .subpackets
                .iter()
                .map(|p| p.sum_versions())
                .sum::<u16>()
    }

    fn value(&self) -> u64 {
        let mut subvalues = self.subpackets.iter().map(|p| p.value());
        match u8::from(self.id) {
            0 => subvalues.sum(),
            1 => subvalues.product(),
            2 => subvalues.min().unwrap(),
            3 => subvalues.max().unwrap(),
            4 => self.literal,
            5 => {
                if subvalues.next().unwrap() > subvalues.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            6 => {
                if subvalues.next().unwrap() < subvalues.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            7 => {
                if subvalues.next().unwrap() == subvalues.next().unwrap() {
                    1
                } else {
                    0
                }
            }
            _ => {
                panic!("unknown id: {:?}", self.id)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    #[test]
    fn test_literal_packet() {
        let input = "D2FE28";
        let p = Packet::parse_hex(input);

        assert_eq!(p.version, u3::new(6));
        assert_eq!(p.id, u3::new(4));
        assert_eq!(p.literal, 2021);
    }

    #[test]
    fn test_operator_packet1() {
        let input = "38006F45291200";
        let p = Packet::parse_hex(input);

        assert_eq!(p.version, u3::new(1));
        assert_eq!(p.id, u3::new(6));
        assert_eq!(p.subpackets.len(), 2);

        let s = p.subpackets.get(0).unwrap();
        assert_eq!(s.literal, 10);

        let s = p.subpackets.get(1).unwrap();
        assert_eq!(s.literal, 20);
    }

    #[test]
    fn test_operator_packet2() {
        let input = "EE00D40C823060";
        let p = Packet::parse_hex(input);

        assert_eq!(p.version, u3::new(7));
        assert_eq!(p.id, u3::new(3));
        assert_eq!(p.subpackets.len(), 3);

        let s = p.subpackets.get(0).unwrap();
        assert_eq!(s.literal, 1);

        let s = p.subpackets.get(1).unwrap();
        assert_eq!(s.literal, 2);

        let s = p.subpackets.get(2).unwrap();
        assert_eq!(s.literal, 3);
    }

    #[test]
    fn test_operator_packet3() {
        let input = "8A004A801A8002F478";
        let p = Packet::parse_hex(input);

        assert_eq!(p.version, u3::new(4));
        assert_eq!(p.subpackets.len(), 1);

        let s = p.subpackets.get(0).unwrap();
        assert_eq!(s.version, u3::new(1));

        let s = s.subpackets.get(0).unwrap();
        assert_eq!(s.version, u3::new(5));

        let s = s.subpackets.get(0).unwrap();
        assert_eq!(s.id, u3::new(4));
        assert_eq!(s.version, u3::new(6));

        assert_eq!(p.sum_versions(), 16);
    }

    #[test]
    fn test_operator_packet4() {
        let input = "620080001611562C8802118E34";
        let p = Packet::parse_hex(input);

        assert_eq!(p.version, u3::new(3));
        assert_eq!(p.subpackets.len(), 2);

        let s = p.subpackets.get(0).unwrap();
        assert_eq!(s.subpackets.get(0).unwrap().id, u3::new(4));

        let s = p.subpackets.get(1).unwrap();
        assert_eq!(s.subpackets.get(0).unwrap().id, u3::new(4));

        assert_eq!(p.sum_versions(), 12);
    }

    #[test]
    fn test_operator_packet5() {
        let input = "C0015000016115A2E0802F182340";
        let p = Packet::parse_hex(input);

        assert_eq!(p.subpackets.len(), 2);

        let s = p.subpackets.get(0).unwrap();
        assert_eq!(s.subpackets.get(0).unwrap().id, u3::new(4));

        let s = p.subpackets.get(1).unwrap();
        assert_eq!(s.subpackets.get(0).unwrap().id, u3::new(4));

        assert_eq!(p.sum_versions(), 23);
    }

    #[test]
    fn test_operator_packet6() {
        let input = "A0016C880162017C3686B18A3D4780";
        let p = Packet::parse_hex(input);

        assert_eq!(p.subpackets.len(), 1);

        let s = p.subpackets.get(0).unwrap();
        assert_eq!(s.subpackets.len(), 1);

        let s = s.subpackets.get(0).unwrap();
        assert_eq!(s.subpackets.len(), 5);

        assert_eq!(p.sum_versions(), 31);
    }

    #[test]
    fn test_expressions() {
        let testcases = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];
        for testcase in testcases {
            let p = Packet::parse_hex(testcase.0);
            assert_eq!(p.value(), testcase.1);
        }
    }

    #[test]
    fn test_input_own() {
        let (part1_answer, part2_answer) = run(include_str!("../input"));
        assert_eq!(part1_answer, 943);
        assert_eq!(part2_answer, 167737115857);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../input");
        b.iter(|| run(input));
    }
}
