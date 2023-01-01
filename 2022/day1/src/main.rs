#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
    #[test]
    fn total_calories_elf_carrying_confirm() {
        assert_eq!(24000, total_calories_elf_carrying(TEST_INPUT));
    }

    #[test]
    fn total_calories_top_3_elves_carrying_confirm() {
        assert_eq!(45000, total_calories_of_top_x_elves(TEST_INPUT, 3));
    }
}

fn total_calories_elf_carrying(input: &str) -> u32 {
    let elves = input.split("\n\n");
    let mut elves_carrying = elves
        .map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    let iter = elves_carrying.iter_mut();
    iter.max_by(|x, y| x.cmp(y)).unwrap().clone()
}

fn total_calories_of_top_x_elves(input: &str, x: usize) -> u32 {
    let elves_carrying = total_calories_each_elf_carrying(input);
    elves_carrying.iter().take(x).sum::<u32>()
}

fn total_calories_each_elf_carrying(input: &str) -> Vec<u32> {
    let elves = input.split("\n\n");
    let mut elves_carrying = elves
        .map(|e| e.lines().map(|l| l.parse::<u32>().unwrap()).sum())
        .collect::<Vec<u32>>();
    elves_carrying.sort_by(|x, y| y.cmp(x));
    elves_carrying
}
fn main() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    println!("{}", total_calories_elf_carrying(&input));
    println!("{}", total_calories_of_top_x_elves(&input, 3));
    Ok(())
}
