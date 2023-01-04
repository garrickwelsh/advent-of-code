use std::{cmp::min, collections::HashMap};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, digit0},
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
};
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn line_parse_test() {
        const LINE1: &str = "aaaaa-bbb-z-y-x-123[abxyz]";
        const LINE2: &str = "a-b-c-d-e-f-g-h-987[abcde]";
        const LINE3: &str = "not-a-real-room-404[oarel]";
        const LINE4: &str = "totally-real-room-200[decoy]";

        line_parse_test_impl(LINE1, true);
        line_parse_test_impl(LINE2, true);
        line_parse_test_impl(LINE3, true);
        line_parse_test_impl(LINE4, false);
    }

    #[test]
    fn shift_ciper_test() {
        let input = "qzmt-zixmtkozy-ivhz";
        let sector_id: u32 = 343;
        let expected = "very-encrypted-name";
        assert_eq!(expected, simple_map(input, sector_id));
    }

    fn line_parse_test_impl(line: &str, valid: bool) {
        dbg!(get_record_valid_and_sector(line).unwrap());
        assert_eq!(valid, get_record_valid_and_sector(line).unwrap().1.valid);
    }
}

#[derive(Debug)]
struct RecordValid {
    sector_id: u32,
    valid: bool,
}

#[derive(Debug)]
struct ScoreResult {
    score: u32,
    letter: char,
}

#[derive(Debug, Serialize)]
struct Message<'a> {
    words: Vec<&'a str>,
    sector_id: u32,
    checksum: &'a str,
}

fn print_records<'a>(lines: &'a [&'a str]) -> nom::IResult<&'a str, ()> {
    let mut messages = Vec::<Message>::new();
    for line in lines {
        let (_, result) = tuple((
            separated_list0(tag("-"), nom::character::complete::alpha1),
            preceded(tag("-"), digit0),
            delimited(tag("["), alpha0, tag("]")),
        ))(*line)?;

        let words = result.0;
        let checksum = result.2;
        let sector_id = result.1.parse::<u32>().unwrap();
        let message = Message {
            words,
            checksum,
            sector_id,
        };
        messages.push(message);
    }
    let json = serde_json::to_string(messages.as_slice()).unwrap();
    println!("{}", json);
    // dbg!(messages);
    Ok(("", ()))
}

fn get_record_valid_and_sector(line: &str) -> nom::IResult<&str, RecordValid> {
    let (remaining, result) = tuple((
        separated_list0(tag("-"), nom::character::complete::alpha1),
        preceded(tag("-"), digit0),
        delimited(tag("["), alpha0, tag("]")),
    ))(line)?;
    let to_score = result.0;
    let sector_id = result.1.parse::<u32>().unwrap();
    let checksum = result.2;
    let mut hm = HashMap::<char, u32>::new();
    let mut calculator = Vec::<ScoreResult>::new();
    for i in to_score {
        for j in i.chars() {
            let ck = !hm.contains_key(&j);
            if ck {
                hm.insert(j, 0);
            }
            *hm.get_mut(&j).unwrap() += 1;
        }
    }
    for (k, v) in hm {
        calculator.push(ScoreResult {
            score: v,
            letter: k,
        });
    }
    calculator.sort_by(|a, b| {
        a.score
            .cmp(&b.score)
            .reverse()
            .then(a.letter.cmp(&b.letter))
    });
    let mut calculated_checksum = String::new();
    for i in calculator[0..min(5, calculator.len())].iter() {
        calculated_checksum.push(i.letter);
    }
    Ok((
        remaining,
        RecordValid {
            sector_id,
            valid: &calculated_checksum == checksum,
        },
    ))
}

fn simple_map(line: &str, rotate: u32) -> String {
    let rotate = rotate % 26;
    let mut retval = String::with_capacity(line.len());
    for c in line.chars() {
        let mut m = c;
        for _ in 0..rotate {
            m = match m {
                'a' => 'b',
                'b' => 'c',
                'c' => 'd',
                'd' => 'e',
                'e' => 'f',
                'f' => 'g',
                'g' => 'h',
                'h' => 'i',
                'i' => 'j',
                'j' => 'k',
                'k' => 'l',
                'l' => 'm',
                'm' => 'n',
                'n' => 'o',
                'o' => 'p',
                'p' => 'q',
                'q' => 'r',
                'r' => 's',
                's' => 't',
                't' => 'u',
                'u' => 'v',
                'v' => 'w',
                'w' => 'x',
                'x' => 'y',
                'y' => 'z',
                'z' => 'a',
                _ => m,
            };
        }
        retval.push(m);
    }
    retval
}

fn main() {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    // Check out against unit test data
    // const LINE1: &str = "aaaaa-bbb-z-y-x-123[abxyz]";
    // const LINE2: &str = "a-b-c-d-e-f-g-h-987[abcde]";
    // const LINE3: &str = "not-a-real-room-404[oarel]";
    // const LINE4: &str = "totally-real-room-200[decoy]";
    // println!("{:?}", get_record_valid_and_sector(LINE1).unwrap().1);
    // println!("{:?}", get_record_valid_and_sector(LINE2).unwrap().1);
    // println!("{:?}", get_record_valid_and_sector(LINE3).unwrap().1);
    // println!("{:?}", get_record_valid_and_sector(LINE4).unwrap().1);
    // const LINES: [&str; 4] = [LINE1, LINE2, LINE3, LINE4];
    // print_records(&LINES).unwrap();

    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let lines = lines.map(|l| l.unwrap()).collect::<Vec<String>>();
    let mut lines_slices = Vec::<&str>::new();
    for l in lines.iter() {
        lines_slices.push(l.as_str());
    }

    let mut line_details = Vec::<(String, RecordValid)>::with_capacity(lines.len());
    for l in lines {
        let record = get_record_valid_and_sector(l.as_str()).unwrap().1;
        let mapped_string = simple_map(l.as_str(), record.sector_id);
        line_details.push((mapped_string, record));
    }
    println!(
        "{}",
        line_details
            .iter()
            .filter(|r| r.1.valid)
            .map(|r| r.1.sector_id)
            .sum::<u32>()
    );
    let result = line_details
        .iter()
        .filter(|r| r.0.find("northpole").is_some())
        .map(|r| r.1.sector_id)
        .collect::<Vec<u32>>();
    println!("{:?}", result[0]);
    // println!("{:?}", line_details); //.map(|d| d.0).collect::<Vec<String>>());
}
