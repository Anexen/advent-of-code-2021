use std::ops::ControlFlow;

use bitvec::prelude::*;

const LITERAL_PACKET_TYPE_ID: u8 = 4;

#[derive(Debug)]
enum PacketValue {
    Literal(u64),
    Operator(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    value: PacketValue,
}

fn read_packet(bits: &BitSlice<Msb0, u8>) -> (Packet, usize) {
    let version: u8 = bits[0..3].load_be();
    let type_id: u8 = bits[3..6].load_be();

    println!("New packet {}:{} [{}]", version, type_id, bits.len());

    if type_id == LITERAL_PACKET_TYPE_ID {
        let r = bits[6..]
            .chunks(5)
            .try_fold(BitVec::<Msb0, u8>::new(), |mut acc, x| {
                acc.extend_from_bitslice(&x[1..5]);
                if x[0] {
                    ControlFlow::Continue(acc)
                } else {
                    ControlFlow::Break(acc)
                }
            });

        if let ControlFlow::Break(p) = r {
            let length = 6 + p.len() + p.len() / 4;
            return (
                Packet {
                    version,
                    type_id,
                    value: PacketValue::Literal(p.load_be()),
                },
                length,
            );
        } else {
            unreachable!()
        }
    } else {
        let length_type_id: u8 = bits[6] as u8;

        if length_type_id == 0 {
            let length: u16 = bits[7..22].load_be();

            println!("{}", length);
            let mut bits_read_total = 0;
            let mut sub_packets = Vec::new();
            loop {
                let (p, bits_read) = read_packet(&bits[22 + bits_read_total..]);
                println!("bits read: {}", bits_read);
                sub_packets.push(p);
                bits_read_total += bits_read;
                if bits_read_total >= length.into() {
                    break;
                }
            }
            return (
                Packet {
                    version,
                    type_id,
                    value: PacketValue::Operator(sub_packets),
                },
                22 + bits_read_total,
            );
        } else {
            let count: u16 = bits[7..18].load_be();
            println!("{}", count);

            let mut bits_read_total = 0;
            let mut sub_packets = Vec::new();

            for _ in 0..count {
                let (p, bits_read) = read_packet(&bits[18 + bits_read_total..]);
                sub_packets.push(p);
                bits_read_total += bits_read;
            }

            return (
                Packet {
                    version,
                    type_id,
                    value: PacketValue::Operator(sub_packets),
                },
                18 + bits_read_total,
            );
        }
    }
}

fn read_input(input: &str) -> BitVec<Msb0, u8> {
    input.trim_end().chars().enumerate().fold(
        BitVec::<Msb0, u8>::repeat(false, input.len() * 4),
        |mut bits, (i, x)| {
            bits[i * 4..i * 4 + 4].store(x.to_digit(16).unwrap());
            bits
        },
    )
}

fn count_versions(packet: &Packet, count: u64) -> u64 {
    match &packet.value {
        PacketValue::Literal(_) => return count + packet.version as u64,
        PacketValue::Operator(sub_packets) => {
            return count
                + packet.version as u64
                + sub_packets
                    .iter()
                    .map(|sp| count_versions(sp, count))
                    .sum::<u64>()
        }
    }
}

fn eval_packet(packet: &Packet) -> u64 {
    match &packet.value {
        PacketValue::Literal(x) => return *x,
        PacketValue::Operator(sub_packets) => match packet.type_id {
            0 => sub_packets.iter().map(eval_packet).sum::<u64>(),
            1 => sub_packets.iter().map(eval_packet).product::<u64>(),
            2 => sub_packets.iter().map(eval_packet).min().unwrap(),
            3 => sub_packets.iter().map(eval_packet).max().unwrap(),
            5 => (eval_packet(&sub_packets[0]) > eval_packet(&sub_packets[1])) as u64,
            6 => (eval_packet(&sub_packets[0]) < eval_packet(&sub_packets[1])) as u64,
            7 => (eval_packet(&sub_packets[0]) == eval_packet(&sub_packets[1])) as u64,
            _ => unreachable!(),
        },
    }
}

pub fn part_a(input: Option<&str>) -> u64 {
    let bits = read_input(input.unwrap_or(include_str!("../input.txt")));
    let (packet, _) = read_packet(&bits);
    count_versions(&packet, 0)
}

pub fn part_b(input: Option<&str>) -> u64 {
    let bits = read_input(input.unwrap_or(include_str!("../input.txt")));
    let (packet, _) = read_packet(&bits);

    eval_packet(&packet)
}

#[cfg(test)]
mod tests {

    use super::{Packet, PacketValue};

    fn assert_literal_value(p: &Packet, x: u64) {
        match p.value {
            PacketValue::Literal(a) => assert_eq!(a, x),
            _ => panic!(),
        }
    }
    #[test]
    fn test_literal_packet_works() {
        let (packet, _) = super::read_packet(&super::read_input("D2FE28"));
        assert_literal_value(&packet, 2021);
    }

    #[test]
    fn test_operator_packet_works_length_type_0() {
        let (packet, _) = super::read_packet(&super::read_input("38006F45291200"));
        match packet.value {
            super::PacketValue::Operator(sub_packets) => {
                assert_literal_value(&sub_packets[0], 10);
                assert_literal_value(&sub_packets[1], 20);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_operator_packet_works_length_type_1() {
        let (packet, _) = super::read_packet(&super::read_input("EE00D40C823060"));
        match packet.value {
            super::PacketValue::Operator(sub_packets) => {
                for i in 0..sub_packets.len() {
                    assert_literal_value(&sub_packets[i], 1 + i as u64);
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_nested() {
        let (packet, _) = super::read_packet(&super::read_input("8A004A801A8002F478"));

        assert_eq!(packet.version, 4);

        match packet.value {
            PacketValue::Operator(p1) => {
                assert_eq!(p1[0].version, 1);
                match &p1[0].value {
                    PacketValue::Operator(p2) => {
                        assert_eq!(p2[0].version, 5);
                        match &p2[0].value {
                            PacketValue::Operator(p3) => {
                                assert_eq!(p3[0].version, 6);
                                match p3[0].value {
                                    PacketValue::Literal(_) => {}
                                    _ => panic!(),
                                }
                            }
                            _ => panic!(),
                        }
                    }
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_part_a_works() {
        assert_eq!(super::part_a(Some("8A004A801A8002F478")), 16);
        assert_eq!(super::part_a(Some("620080001611562C8802118E34")), 12);
        assert_eq!(super::part_a(Some("C0015000016115A2E0802F182340")), 23);
        assert_eq!(super::part_a(Some("A0016C880162017C3686B18A3D4780")), 31);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(None), 893);
    }

    #[test]
    fn test_part_b_works() {
        assert_eq!(super::part_b(Some("C200B40A82")), 3);
        assert_eq!(super::part_b(Some("04005AC33890")), 54);
        assert_eq!(super::part_b(Some("880086C3E88112")), 7);
        assert_eq!(super::part_b(Some("CE00C43D881120")), 9);
        assert_eq!(super::part_b(Some("D8005AC2A8F0")), 1);
        assert_eq!(super::part_b(Some("F600BC2D8F")), 0);
        assert_eq!(super::part_b(Some("9C005AC2F8F0")), 0);
        assert_eq!(super::part_b(Some("9C0141080250320F1802104A08")), 1);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(super::part_b(None), 4358595186090);
    }
}
