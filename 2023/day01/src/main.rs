#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(get_number("1abc2"), 12);
        assert_eq!(get_number("pqr3stu8vwx"), 38);
        assert_eq!(get_number("a1b2c3d4e5f"), 15);
        assert_eq!(get_number("treb7uchet"), 77);
    }

    #[test]
    fn parse_input() {
        const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let mut s = 0;
        for line in INPUT.lines() {
            s += get_number(line);
        }
        assert_eq!(s, 142);
    }
}

fn get_number(line: &str) -> u64 {
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
        answer += get_number(line);
    }
    println!("The answer is {0}", answer);
}
