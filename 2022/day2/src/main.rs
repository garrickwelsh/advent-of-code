#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn win_lose_score_test() {
        let lines = INPUT.split("\n");
        assert_eq!(
            15u32,
            lines.map(|l| rock_paper_scissors_play_game_part1(l)).sum()
        )
    }

    #[test]
    fn rock_paper_scissors_get_characters_test() {
        let mut lines = INPUT.split("\n");
        let line = lines.next().unwrap();
        assert_eq!((ROCK, PAPER), rock_paper_scissors_get_characters(line));
        let line = lines.next().unwrap();
        assert_eq!((PAPER, ROCK), rock_paper_scissors_get_characters(line));
        let line = lines.next().unwrap();
        assert_eq!(
            (SCISSORS, SCISSORS),
            rock_paper_scissors_get_characters(line)
        );
    }

    #[test]
    fn rock_paper_scissors_test_part2() {
        let lines = INPUT.split("\n");
        assert_eq!(
            12u32,
            lines.map(|l| rock_paper_scissors_play_game_part2(l)).sum()
        )
    }
}

const ROCK: char = 'r';
const PAPER: char = 'p';
const SCISSORS: char = 's';

fn rock_paper_scissors_play_game_part1(line_input: &str) -> u32 {
    rock_paper_scissors_score_game(rock_paper_scissors_get_characters(line_input))
}
fn rock_paper_scissors_play_game_part2(line_input: &str) -> u32 {
    let (elf_chose, score) = rock_paper_scissors_get_characters_encoded(line_input);
    let you_choose = calculate_rock_paper_scissors(elf_chose, score);
    rock_paper_scissors_score_game((elf_chose, you_choose))
}

fn rock_paper_scissors_score_game(game: (char, char)) -> u32 {
    rock_paper_scissors_win_lose_score(game) + rock_paper_scissors_score(game.1)
}

fn rock_paper_scissors_score(rps: char) -> u32 {
    match rps {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
        _ => panic!(),
    }
}

fn rock_paper_scissors_win_lose_score(game: (char, char)) -> u32 {
    match game {
        (ROCK, ROCK) => 3,
        (ROCK, PAPER) => 6,
        (ROCK, SCISSORS) => 0,
        (PAPER, ROCK) => 0,
        (PAPER, PAPER) => 3,
        (PAPER, SCISSORS) => 6,
        (SCISSORS, ROCK) => 6,
        (SCISSORS, PAPER) => 0,
        (SCISSORS, SCISSORS) => 3,
        _ => panic!(),
    }
}

fn rock_paper_scissors_get_characters(line_input: &str) -> (char, char) {
    use nom::character::complete::anychar;
    use nom::character::complete::one_of;
    let (remaining, elf_chose) =
        one_of::<_, _, (&str, nom::error::ErrorKind)>("ABC")(line_input).unwrap();
    let (remaining, _) = anychar::<_, (&str, nom::error::ErrorKind)>(remaining).unwrap();
    let (_, me_chose) = one_of::<_, _, (&str, nom::error::ErrorKind)>("XYZ")(remaining).unwrap();
    let elf_chose = match elf_chose {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        _ => panic!(),
    };
    let me_chose = match me_chose {
        'X' => ROCK,
        'Y' => PAPER,
        'Z' => SCISSORS,
        _ => panic!(),
    };
    (elf_chose, me_chose)
}
fn rock_paper_scissors_get_characters_encoded(line_input: &str) -> (char, u32) {
    use nom::character::complete::anychar;
    use nom::character::complete::one_of;
    let (remaining, elf_chose) =
        one_of::<_, _, (&str, nom::error::ErrorKind)>("ABC")(line_input).unwrap();
    let (remaining, _) = anychar::<_, (&str, nom::error::ErrorKind)>(remaining).unwrap();
    let (_, me_chose) = one_of::<_, _, (&str, nom::error::ErrorKind)>("XYZ")(remaining).unwrap();
    let elf_chose = match elf_chose {
        'A' => ROCK,
        'B' => PAPER,
        'C' => SCISSORS,
        _ => panic!(),
    };
    let me_chose = match me_chose {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!(),
    };
    (elf_chose, me_chose)
}

fn calculate_rock_paper_scissors(elf_chose: char, outcome: u32) -> char {
    match elf_chose {
        ROCK => match outcome {
            0 => SCISSORS,
            3 => ROCK,
            6 => PAPER,
            _ => panic!(),
        },
        PAPER => match outcome {
            0 => ROCK,
            3 => PAPER,
            6 => SCISSORS,
            _ => panic!(),
        },
        SCISSORS => match outcome {
            0 => PAPER,
            3 => SCISSORS,
            6 => ROCK,
            _ => panic!(),
        },
        _ => panic!(),
    }
}

fn main() -> std::io::Result<()> {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();
    println!(
        "{}",
        lines
            .map(|l| rock_paper_scissors_play_game_part1(&l.unwrap()))
            .sum::<u32>()
    );
    let file = File::open(&path)?;
    let lines = io::BufReader::new(file).lines();
    println!(
        "{}",
        lines
            .map(|l| rock_paper_scissors_play_game_part2(&l.unwrap()))
            .sum::<u32>()
    );
    Ok(())
}
