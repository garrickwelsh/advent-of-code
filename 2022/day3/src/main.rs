#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn rucksack_common_sum() {
        let lines = INPUT.lines();
        let res = lines
            .map(|l| {
                let mut result: [bool; 53] = [false; 53];
                common_items_in_compartment(l, &mut result);
                score_common_items(&result)
            })
            .sum();
        assert_eq!(157u32, res);
    }

    #[test]
    fn rucksack_common_split_value_check_line1() {
        const LINE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let mut result: [bool; 53] = [false; 53];
        common_items_in_compartment(LINE_INPUT, &mut result);
        assert_eq!(16u32, score_common_items(&result));
    }
    #[test]
    fn rucksack_common_split_value_check_line2() {
        const LINE_INPUT: &str = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let mut result: [bool; 53] = [false; 53];
        common_items_in_compartment(LINE_INPUT, &mut result);
        assert_eq!(38u32, score_common_items(&result));
    }

    #[test]
    fn rucksack_common_split_check() {
        const LINE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";
        assert_eq!(
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            items_in_compartments(LINE_INPUT)
        );
    }
}

const fn value_map() -> [usize; 128] {
    let mut retval = [0usize; 128];
    let mut i = 0;
    while i < 128 {
        if i >= 65 && i < 91 {
            retval[i] = i - 64 + 26;
        } else if i >= 97 && i < 123 {
            retval[i] = i - 96;
        }
        i += 1;
    }
    retval
}

fn score_common_items(result: &[bool; 53]) -> u32 {
    result
        .iter()
        .enumerate()
        .map(|(i, v)| if *v { i as u32 } else { 0u32 })
        .sum::<u32>()
}

fn common_items_in_compartment(line: &str, result: &mut [bool; 53]) {
    let map = value_map();
    let mut c1r = [0u32; 53];
    let mut c2r = [0u32; 53];
    let (c1, c2) = items_in_compartments(line);
    let c1b = c1.as_bytes();
    let c2b = c2.as_bytes();

    c1b.iter().for_each(|b| {
        let i = map[(*b as usize)];
        c1r[i as usize] += 1;
    });
    c2b.iter().for_each(|b| {
        let i = map[(*b as usize)];
        c2r[i as usize] += 1;
    });

    for i in 0..53 {
        result[i] = (c1r[i] > 0) && (c2r[i] > 0);
    }
}

#[inline]
fn items_in_compartments(line: &str) -> (&str, &str) {
    let middle = line.len() / 2;
    line.split_at(middle)
}

fn main() -> std::io::Result<()> {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();
    let res = lines
        .map(|l| {
            let mut result: [bool; 53] = [false; 53];
            common_items_in_compartment(&l.unwrap(), &mut result);
            score_common_items(&result)
        })
        .sum::<u32>();
    println!("{}", res);
    Ok(())
}
