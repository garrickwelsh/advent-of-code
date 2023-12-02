#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(get_number_simple("1abc2"), 12);
        assert_eq!(get_number_simple("pqr3stu8vwx"), 38);
        assert_eq!(get_number_simple("a1b2c3d4e5f"), 15);
        assert_eq!(get_number_simple("treb7uchet"), 77);
    }

    #[test]
    fn parse_input() {
        const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let mut answer = 0;
        for line in INPUT.lines() {
            answer += get_number_simple(line);
        }
        assert_eq!(answer, 142);
    }

    #[test]
    fn test_parse_line2() {
        assert_eq!(get_number("two1nine"), 29);
        assert_eq!(get_number("eightwothree"), 83);
        assert_eq!(get_number("abcone2threexyz"), 13);
        assert_eq!(get_number("xtwone3four"), 24);
        assert_eq!(get_number("4nineeightseven2"), 42);
        assert_eq!(get_number("zoneight234"), 14);
        assert_eq!(get_number("7pqrstsixteen"), 76);
    }

    #[test]
    fn parse_test2() {
        const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let mut answer = 0;
        for line in INPUT.lines() {
            answer += get_number(line);
        }
        assert_eq!(answer, 281);
    }
}

fn match_digit(number: &str) -> char {
    match number {
        "0" => '0',
        "1" => '1',
        "2" => '2',
        "3" => '3',
        "4" => '4',
        "5" => '5',
        "6" => '6',
        "7" => '7',
        "8" => '8',
        "9" => '9',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => panic!(),
    }
}

fn get_number(line: &str) -> u64 {
    const NUMBERS: [&str; 19] = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];

    let test: [Vec<(usize, &str)>; 19] =
        NUMBERS.map(|search_for| line.match_indices(search_for).collect());

    let mut number_map = Vec::new();
    for i in test {
        for j in i {
            number_map.push(j);
        }
    }

    number_map.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let first = number_map.first().unwrap();
    let last = number_map.last().unwrap();

    let f = match_digit(first.1);
    let l = match_digit(last.1);

    let mut num_digits = String::new();
    num_digits.push(f);
    num_digits.push(l);

    num_digits.parse().unwrap()
}

fn get_number_simple(line: &str) -> u64 {
    let mut first: Option<char> = None;
    let mut last: Option<char> = None;
    for c in line.chars() {
        match (c.is_digit(10), first.is_some(), last.is_some()) {
            (true, false, false) => {
                first = Some(c);
                last = Some(c);
            }
            (true, true, true) => last = Some(c),
            (_, _, _) => (),
        }
    }
    if let (Some(f), Some(l)) = (first, last) {
        let mut s = String::new();
        s.push(f);
        s.push(l);
        return s.parse().unwrap();
    } else {
        panic!();
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

    let mut answer = 0;
    for line in input.lines() {
        answer += get_number_simple(line);
    }

    println!("The part 1 answer is {0}", answer);
    let mut answer = 0;
    for line in input.lines() {
        answer += get_number(line);
    }
    println!("The part 2 answer is {0}", answer);
}
