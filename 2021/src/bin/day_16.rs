use aoc2021::*;
use nom::bits::complete::{tag, take};
use nom::branch::alt;
use nom::combinator::{map, map_opt, verify};
use nom::multi::{length_count, many_till};
use nom::sequence::{preceded, tuple};
use nom::{bits, IResult};
use std::cmp::min;

aoc_main!(
    day: 16,
    test_input: r#"9C0141080250320F1802104A08"#,
    parser: parse,
    task_1: task_1,
    expected_1: 20,
    task_2: task_2,
    expected_2: 1,
);

fn parse(raw_input: &str) -> Result<Vec<u8>> {
    (0..raw_input.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&raw_input[i..i + 2], 16).map_err(|_e| {
                anyhow!(
                    "Unable to parse hex string {} into byte",
                    &raw_input[i..i + 2]
                )
            })
        })
        .collect::<Result<Vec<u8>>>()
}

fn task_1(input: &[u8]) -> Result<u64> {
    let packet = parse_packet(input);

    match packet {
        Ok((_i, packet)) => Ok(sum_versions(&packet)),
        Err(_e) => Err(anyhow!("Could not decode packet due to error")),
    }
}

fn task_2(input: &[u8]) -> Result<u64> {
    let packet = parse_packet(input);

    match packet {
        Ok((_i, packet)) => Ok(eval(&packet)),
        Err(_e) => Err(anyhow!("Could not decode packet due to error")),
    }
}

fn sum_versions(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { version, .. } => *version as u64,
        Packet::Operator {
            version,
            subpackets,
            ..
        } => {
            let mut sum = *version as u64;
            for subpacket in subpackets {
                sum += sum_versions(subpacket);
            }

            sum
        }
    }
}

fn eval(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal { number, .. } => *number as u64,
        Packet::Operator {
            operator,
            subpackets,
            ..
        } => eval_operator(operator, subpackets.iter().map(eval)).unwrap_or_else(|| {
            panic!(
                "Invalid operation: {:?} with {} subpackets",
                operator,
                subpackets.len()
            )
        }),
    }
}

fn eval_operator<It: Iterator<Item = u64>>(
    operator: &Operator,
    subpacket_values: It,
) -> Option<u64> {
    match operator {
        Operator::Sum => Some(subpacket_values.sum()),
        Operator::Product => Some(subpacket_values.product()),
        Operator::Minimum => subpacket_values.min(),
        Operator::Maximum => subpacket_values.max(),
        Operator::GreaterThan => subpacket_values
            .tuple_windows()
            .map(|(a, b)| (a > b) as u64)
            .next(),
        Operator::LessThan => subpacket_values
            .tuple_windows()
            .map(|(a, b)| (a < b) as u64)
            .next(),
        Operator::EqualTo => subpacket_values
            .tuple_windows()
            .map(|(a, b)| (a == b) as u64)
            .next(),
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal {
        version: u8,
        number: u64,
    },
    Operator {
        version: u8,
        operator: Operator,
        subpackets: Vec<Packet>,
    },
}

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operator {
    fn try_from_type_id(type_id: u8) -> Option<Operator> {
        match type_id {
            0 => Some(Operator::Sum),
            1 => Some(Operator::Product),
            2 => Some(Operator::Minimum),
            3 => Some(Operator::Maximum),
            5 => Some(Operator::GreaterThan),
            6 => Some(Operator::LessThan),
            7 => Some(Operator::EqualTo),
            _ => None,
        }
    }
}

fn version(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    // Version is a u8 from 3 bits
    take(3usize)(i)
}

fn type_id(i: (&[u8], usize)) -> IResult<(&[u8], usize), u8> {
    // Type ID is a u8 from 3 bits
    take(3usize)(i)
}

struct PacketHeader {
    version: u8,
    type_id: u8,
}

fn header(i: (&[u8], usize)) -> IResult<(&[u8], usize), PacketHeader> {
    // A header consists of a version and then a type_id
    map(tuple((version, type_id)), |(version, type_id)| {
        PacketHeader { version, type_id }
    })(i)
}

fn literal_packet(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let group_prefix_one = |i| tag(0x1, 1usize)(i);
    let group_prefix_zero = |i| tag(0x0, 1usize)(i);

    // A single nibble of a number consists of 4 bits
    let nibble = |i| take::<_, u8, _, _>(4usize)(i);

    // The whole number consists of 0 or more nibbles.
    // We keep parsing nibbles while we see a nibble preceded by a `group_prefix_one`.
    // When we see `group_prefix_zero`, we parse one more nibble.
    // We then add this last nibble to the nibbles vector.
    let nibbles = move |i| {
        map(
            many_till(
                preceded(group_prefix_one, nibble),
                preceded(group_prefix_zero, nibble),
            ),
            |(mut nibbles, last_nibble)| {
                nibbles.push(last_nibble);
                nibbles
            },
        )(i)
    };

    // Collect little-endian nibbles into a u64 (i.e. nibbles[0] represents the lower 4 bits of the u64).
    // Nibbles are represented by a Vec<u8> where
    // only the least-significant byte of each byte in the vector is considered.
    // A variable length u64 consists of a _maximum_ of 64 / 4 = 16 nibbles.
    let variable_length_u64 = move |i| {
        map(nibbles, |parts| {
            let mut number = 0u64;

            for &part in &parts[..min(16, parts.len())] {
                number <<= 4;
                number |= (part & 0x0F) as u64;
            }

            number
        })(i)
    };

    // A literal packet is a packet that consists of a header and then
    // a variable_length u64. The type_id should always be 4.
    map(
        tuple((
            verify(header, |header| header.type_id == 4),
            variable_length_u64,
        )),
        |(header, number)| Packet::Literal {
            version: header.version,
            number,
        },
    )(i)
}

fn operator_packet(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    // Read total length (15 bits), then parse subpackets while length of subpackets read so
    // far is less than the total length.
    let subpackets_from_length = |i| {
        let (i, subpacket_total_length) = take(15usize)(i)?;

        let mut length = 0u16;
        let mut packets = vec![];
        let mut remaining_i = i;

        while length < subpacket_total_length {
            let (new_remaining_i, packet) = packet(remaining_i)?;
            length += consumed_bits(remaining_i, new_remaining_i) as u16;
            remaining_i = new_remaining_i;

            packets.push(packet);
        }

        Ok((remaining_i, packets))
    };

    // Read an 11 bit number, then apply the `packet` parser that number of times
    let subpackets_from_total_number = length_count(take::<_, u16, _, _>(11usize), packet);

    let length_of_subpackets_specified = preceded(tag(0x0, 1usize), subpackets_from_length);
    let num_subpackets_specified = preceded(tag(0x1, 1usize), subpackets_from_total_number);

    // Subpackets are either encoded with the number of bits specified OR the number of subpackets
    // specified.
    let subpackets = alt((length_of_subpackets_specified, num_subpackets_specified));

    // An operator packet consists of a header, and then a number of subpackets.
    // We verify that the type_id is NOT equal to 4 (because then it would be a literal packet)
    let mut parse = map_opt(
        tuple((verify(header, |header| header.type_id != 4), subpackets)),
        |(header, packets)| {
            Some(Packet::Operator {
                version: header.version,
                operator: Operator::try_from_type_id(header.type_id)?,
                subpackets: packets,
            })
        },
    );

    parse(i)
}

fn consumed_bits(input_a: (&[u8], usize), input_b: (&[u8], usize)) -> usize {
    // Calculate the number of consumed bits between two bit inputs (input_a - input_b).
    let (input_a, bit_idx_a) = input_a;
    let (input_b, bit_idx_b) = input_b;

    (input_a.len() * 8 - bit_idx_a) - (input_b.len() * 8 - bit_idx_b)
}

fn packet(i: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    // A packet can either be a literal packet or an operator packet
    alt((literal_packet, operator_packet))(i)
}

fn parse_packet(i: &[u8]) -> IResult<&[u8], Packet> {
    // Convert the bytes input into a bits input and parse a single packet.
    // Leftover bits are discarded
    bits(packet)(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_packet() {
        let input = parse("D2FE28").unwrap();

        let packet = literal_packet((&input, 0));

        let expected_packet = Packet::Literal {
            version: 6,
            number: 2021,
        };

        match packet {
            Ok((_i, packet)) => assert_eq!(packet, expected_packet),
            Err(_) => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_operator_packet_with_literal_packets() {
        let input = parse("38006F45291200").unwrap();

        let packet = operator_packet((&input, 0));

        let expected_packet = Packet::Operator {
            version: 1,
            operator: Operator::LessThan,
            subpackets: vec![
                Packet::Literal {
                    version: 6,
                    number: 10,
                },
                Packet::Literal {
                    version: 2,
                    number: 20,
                },
            ],
        };

        match packet {
            Ok((_i, packet)) => assert_eq!(packet, expected_packet),
            Err(_) => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_operator_packet_with_literal_packets_2() {
        let input = parse("EE00D40C823060").unwrap();

        let packet = operator_packet((&input, 0));

        let expected_packet = Packet::Operator {
            version: 7,
            operator: Operator::Maximum,
            subpackets: vec![
                Packet::Literal {
                    version: 2,
                    number: 1,
                },
                Packet::Literal {
                    version: 4,
                    number: 2,
                },
                Packet::Literal {
                    version: 1,
                    number: 3,
                },
            ],
        };

        match packet {
            Ok((_i, packet)) => assert_eq!(packet, expected_packet),
            Err(_) => panic!("Unexpected error"),
        }
    }
}
