#[cfg(test)]
mod test {
    use super::*;

    pub(super) const TEST_INPUT1: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn parse_part1_test() {
        let (_, packets) = parse_packets(TEST_INPUT1).unwrap();
        let valid = vec![
            MsgCmpResult::Valid,
            MsgCmpResult::Valid,
            MsgCmpResult::Invalid,
            MsgCmpResult::Valid,
            MsgCmpResult::Invalid,
            MsgCmpResult::Valid,
            MsgCmpResult::Invalid,
            MsgCmpResult::Invalid,
        ];
        for (i, v) in packets.iter().enumerate() {
            assert_eq!(valid[i], compare_messages(v));
        }
    }

    #[test]
    fn parse_valid_packet_indexes_part1_test() {
        let (_, packets) = parse_packets(TEST_INPUT1).unwrap();
        assert_eq!(13, sum_valid_packet_indexes(&packets));
    }

    #[test]
    fn calculate_decoder_key_part2_test() {
        let (_, mut packets) = parse_packets(TEST_INPUT1).unwrap();
        assert_eq!(140, calculate_decoder_key(&mut packets));
    }
}

use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug, PartialEq, Eq)]
enum Message {
    Integer(u8),
    List(Vec<Message>),
}

#[derive(Debug)]
struct Packet {
    left: Message,
    right: Message,
}

#[derive(Debug, PartialEq)]
enum MsgCmpResult {
    Indeterminate,
    Valid,
    Invalid,
}

fn parse_two_new_lines(input: &str) -> nom::IResult<&str, &str> {
    use nom::bytes::complete::tag;
    let (remaining, r) = tag("\n\n")(input)?;
    Ok((remaining, r))
}

fn parse_message(input: &str) -> nom::IResult<&str, Message> {
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::combinator::peek;

    let mut remaining = input;

    let mut retval = Vec::<Message>::new();

    loop {
        let result: nom::IResult<&str, &str> = tag(",")(remaining);
        if result.is_ok() {
            remaining = result.unwrap().0;
            continue;
        }

        let result: nom::IResult<&str, &str> = tag("[")(remaining);
        if let Ok((remains, _)) = result {
            let m = parse_message(remains)?;
            remaining = m.0;
            retval.push(m.1);
        }

        let result: nom::IResult<&str, &str> = tag("]")(remaining);
        if result.is_ok() {
            remaining = result.unwrap().0;
            return Ok((remaining, Message::List(retval)));
        }

        let result: nom::IResult<&str, &str> = digit1(remaining);
        if result.is_ok() {
            let result = result.unwrap();
            remaining = result.0;
            retval.push(Message::Integer(result.1.parse::<u8>().unwrap()));
        }

        let result: nom::IResult<&str, &str> = peek(tag("\n"))(remaining);
        if result.is_ok() || remaining.len() == 0 {
            return Ok((remaining, Message::List(retval)));
        }
    }
}

fn parse_packet(input: &str) -> nom::IResult<&str, Packet> {
    use nom::character::complete::newline;
    use nom::sequence::separated_pair;

    let (remaining, messages) = separated_pair(parse_message, newline, parse_message)(input)?;
    Ok((
        remaining,
        Packet {
            left: messages.0,
            right: messages.1,
        },
    ))
}

fn parse_packets(input: &str) -> nom::IResult<&str, Vec<Packet>> {
    use nom::multi::separated_list1;

    let (remaining, packets) = separated_list1(parse_two_new_lines, parse_packet)(input)?;

    Ok((remaining, packets))
}

fn compare_message(left: &Message, right: &Message) -> MsgCmpResult {
    let mut retval = MsgCmpResult::Indeterminate;
    match left {
        Message::List(lmessages) => match right {
            Message::List(rmessages) => {
                let mut riter = rmessages.iter();
                for lmsg in lmessages.iter() {
                    let rmsg = riter.next();
                    if rmsg.is_none() {
                        retval = MsgCmpResult::Invalid;
                        break;
                    }
                    retval = compare_message(lmsg, rmsg.unwrap());
                    if retval != MsgCmpResult::Indeterminate {
                        break;
                    }
                }
                let rmsg = riter.next();
                if retval == MsgCmpResult::Indeterminate && rmsg.is_some() {
                    retval = MsgCmpResult::Valid;
                }
                retval
            }
            Message::Integer(rvalue) => {
                let mut rmessage = Vec::<Message>::new();
                rmessage.push(Message::Integer(*rvalue));
                compare_message(left, &Message::List(rmessage))
            }
        },
        Message::Integer(lvalue) => match right {
            Message::List(_) => {
                let mut lmessage = Vec::<Message>::new();
                lmessage.push(Message::Integer(*lvalue));
                compare_message(&Message::List(lmessage), right)
            }
            Message::Integer(rvalue) => {
                if lvalue < rvalue {
                    MsgCmpResult::Valid
                } else if lvalue == rvalue {
                    MsgCmpResult::Indeterminate
                } else {
                    MsgCmpResult::Invalid
                }
            }
        },
    }
}

fn compare_messages(packet: &Packet) -> MsgCmpResult {
    let (left, right) = (&packet.left, &packet.right);
    compare_message(left, right)
}

fn sum_valid_packet_indexes(packets: &Vec<Packet>) -> usize {
    packets
        .iter()
        .map(|p| compare_messages(p))
        .enumerate()
        .filter(|(_, cr)| *cr == MsgCmpResult::Valid)
        .map(|(i, _)| i + 1)
        .sum()
}

fn calculate_decoder_key(packets: &mut Vec<Packet>) -> usize {
    packets.push(Packet {
        left: Message::List(vec![Message::List(vec![Message::Integer(2)])]),
        right: Message::List(vec![Message::List(vec![Message::Integer(6)])]),
    });
    let mut messages = Vec::<&Message>::new();
    for p in packets {
        messages.push(&p.left);
        messages.push(&p.right);
    }
    messages.sort();
    let key1 = Message::List(vec![Message::List(vec![Message::Integer(2)])]);
    let key2 = Message::List(vec![Message::List(vec![Message::Integer(6)])]);

    let mut index1 = 0;
    let mut index2 = 0;

    for (i, m) in messages.into_iter().enumerate() {
        if m == &key1 {
            index1 = i + 1;
        }
        if m == &key2 {
            index2 = i + 1;
        }
    }
    index1 * index2
}

impl Ord for Message {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match compare_message(self, other) {
            MsgCmpResult::Valid => std::cmp::Ordering::Less,
            MsgCmpResult::Invalid => std::cmp::Ordering::Greater,
            MsgCmpResult::Indeterminate => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match compare_message(self, other) {
            MsgCmpResult::Valid => Some(std::cmp::Ordering::Less),
            MsgCmpResult::Invalid => Some(std::cmp::Ordering::Greater),
            MsgCmpResult::Indeterminate => Some(std::cmp::Ordering::Equal),
        }
    }
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    // let (_r, packets) = parse_packets(test::TEST_INPUT1).unwrap();
    // let retval = sum_valid_packet_indexes(&packets);
    // println!("{}", retval);

    // Part 1
    let (_r, packets) = parse_packets(&input).unwrap();
    let retval = sum_valid_packet_indexes(&packets);
    println!("{}", retval);

    let (_r, mut packets) = parse_packets(&input).unwrap();

    let retval = calculate_decoder_key(&mut packets);
    // for i in packets.iter() {
    //     println!("{:?}", compare_messages(i));
    // }
    println!("{}", retval);
}
