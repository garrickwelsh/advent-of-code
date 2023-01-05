#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_for_within_pairs() {
        const LINE: &str = "2-4,6-8";
        let pairs = get_pairs(LINE);
        assert_eq!(false, within_either(&pairs[0], &pairs[1]))
    }

    #[test]
    fn check_for_within_pairs2() {
        run_within_test("2-4,6-8", false);
        run_within_test("2-8,3-7", true);
        run_within_test("2-4,6-8", false);
        run_within_test("2-3,4-5", false);
        run_within_test("5-7,7-9", false);
        run_within_test("2-8,3-7", true);
        run_within_test("6-6,4-6", true);
        run_within_test("2-6,4-8", false);
    }

    #[test]
    fn check_for_overlaps_pairs() {
        run_overlap_test("2-4,6-8", false);
        run_overlap_test("2-3,4-5", false);
        run_overlap_test("5-7,7-9", true);
        run_overlap_test("2-8,3-7", true);
        run_overlap_test("6-6,4-6", true);
        run_overlap_test("2-6,4-8", true);
    }
    fn run_within_test(line: &str, expected_result: bool) {
        let pairs = get_pairs(line);
        assert_eq!(expected_result, within_either(&pairs[0], &pairs[1]))
    }

    fn run_overlap_test(line: &str, expected_result: bool) {
        let pairs = get_pairs(line);
        assert_eq!(expected_result, overlaps_either(&pairs[0], &pairs[1]));
    }
}

#[derive(Debug)]
struct Pair {
    start: u32,
    end: u32,
}

impl Pair {
    fn within(&self, other: &Pair) -> bool {
        other.start >= self.start
            && other.start <= self.end
            && other.end >= self.start
            && other.end <= self.end
            && other.end >= other.start
            && self.end >= self.start
    }

    fn overlaps(&self, other: &Pair) -> bool {
        ((other.start >= self.start && other.start <= self.end)
            || (other.end <= self.end && other.end >= self.start))
            && other.end >= other.start
            && self.end >= self.start
    }
}

fn overlaps_either(pair1: &Pair, pair2: &Pair) -> bool {
    pair1.overlaps(pair2) || pair2.overlaps(pair1)
}

fn within_either(pair1: &Pair, pair2: &Pair) -> bool {
    pair1.within(pair2) || pair2.within(pair1)
}

fn parse(input: &str) -> nom::IResult<&str, (&str, &str, &str, &str)> {
    use nom::character::complete::{char, digit0};
    let (rest, digit1) = digit0(input)?;
    let (rest, _) = char('-')(rest)?;
    let (rest, digit2) = digit0(rest)?;
    let (rest, _) = char(',')(rest)?;
    let (rest, digit3) = digit0(rest)?;
    let (rest, _) = char('-')(rest)?;
    let (rest, digit4) = digit0(rest)?;
    Ok((rest, (digit1, digit2, digit3, digit4)))
}
// fn read_pairs(line: &str, pairs: &[Pairs; 2]) {
fn get_pairs(line: &str) -> [Pair; 2] {
    let output = parse(line).unwrap();
    let pair1 = Pair {
        start: output.1 .0.parse::<u32>().unwrap(),
        end: output.1 .1.parse::<u32>().unwrap(),
    };
    let pair2 = Pair {
        start: output.1 .2.parse::<u32>().unwrap(),
        end: output.1 .3.parse::<u32>().unwrap(),
    };
    // println!("{:?}", output);
    // println!("{:?}", pair1);
    // println!("{:?}", pair2);
    [pair1, pair2]
}

fn main() -> std::io::Result<()> {
    use std::io::prelude::*;
    let path = std::path::Path::new("input.txt");
    let file = std::fs::File::open(&path)?;
    let lines = std::io::BufReader::new(file).lines();
    let within = lines
        .map(|l| {
            let pairs = get_pairs(&l.unwrap());
            if within_either(&pairs[0], &pairs[1]) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    println!("{}", within);

    let file = std::fs::File::open(&path)?;
    let lines = std::io::BufReader::new(file).lines();
    let overlaps = lines
        .map(|l| {
            let pairs = get_pairs(&l.unwrap());
            if overlaps_either(&pairs[0], &pairs[1]) {
                1
            } else {
                0
            }
        })
        .sum::<u32>();
    println!("{}", overlaps);
    Ok(())
}
