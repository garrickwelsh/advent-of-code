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

    #[test]
    fn rucksack_get_badges_check() {
        const GROUP_LINE_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg";
        let final_result = calculate_badge_value(GROUP_LINE_INPUT);
        assert_eq!(18, final_result);
    }
    #[test]
    fn rucksack_get_badges_check2() {
        const GROUP_LINE_INPUT: &str = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let final_result = calculate_badge_value(GROUP_LINE_INPUT);
        assert_eq!(52, final_result);
    }
}

fn calculate_badge_value(lines: &str) -> u32 {
    let mut count1 = [0u32; 53];
    let mut count2 = [0u32; 53];
    let mut count3 = [0u32; 53];
    let mut result = [false; 53];
    let lines = lines.lines();
    let lines = lines.into_iter().collect::<Vec<&str>>();
    items_in_counted(lines[0], &mut count1);
    items_in_counted(lines[1], &mut count2);
    items_in_counted(lines[2], &mut count3);

    for i in 0..53 {
        result[i] = count1[i] > 0 && count2[i] > 0 && count3[i] > 0;
    }
    result
        .iter()
        .enumerate()
        .map(|(i, v)| if *v { i as u32 } else { 0u32 })
        .sum::<u32>()
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
    let mut c1r = [0u32; 53];
    let mut c2r = [0u32; 53];
    let (c1, c2) = items_in_compartments(line);

    items_in_counted(c1, &mut c1r);
    items_in_counted(c2, &mut c2r);

    for i in 0..53 {
        result[i] = (c1r[i] > 0) && (c2r[i] > 0);
    }
}

#[inline]
fn items_in_counted(items: &str, count: &mut [u32; 53]) {
    let bitems = items.as_bytes();

    bitems.iter().for_each(|b| {
        let i = MAP[(*b as usize)];
        count[i as usize] += 1;
    });
}

#[inline]
fn items_in_compartments(line: &str) -> (&str, &str) {
    let middle = line.len() / 2;
    line.split_at(middle)
}

const MAP: [usize; 128] = value_map();

fn main() -> std::io::Result<()> {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");
    // let file = File::open(&path)?;
    // let mut input = String::new();
    // file.read_to_string(&mut input)?;

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

    let file = File::open(&path)?;
    let lines = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let len = lines.len();
    let group_len = len / 3;

    let mut res = 0u32;

    for i in 0..group_len {
        let i1 = i * 3;
        let i2 = i1 + 1;
        let i3 = i1 + 2;
        let mut group = String::new();
        group.push_str(&lines[i1]);
        group.push_str("\n");
        group.push_str(&lines[i2]);
        group.push_str("\n");
        group.push_str(&lines[i3]);
        res += calculate_badge_value(&group);
    }

    println!("{}", res);
    Ok(())
}
