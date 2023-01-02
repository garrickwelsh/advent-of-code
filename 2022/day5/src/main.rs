#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_check_outputs() {
        const INPUT: &str = "move 1 from 2 to 1";
        parse_line_check_given_input_with_outputs(INPUT, 1, 2, 1);
    }

    #[test]
    fn parse_everything_test() {
        const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

        let split = INPUT.split("\n\n").collect::<Vec<&str>>();
        let (_, mut stacks) = parse_crates(split[0]).unwrap();
        split[1].lines().for_each(|l| {
            let (_, result) = parse_moves(l).unwrap();
            move_between_stacks(&mut stacks, result.0, result.1, result.2);
        });
        assert_eq!('C', stacks[0][stacks[0].len() - 1]);
        assert_eq!('M', stacks[1][stacks[1].len() - 1]);
        assert_eq!('Z', stacks[2][stacks[2].len() - 1]);
    }

    fn parse_line_check_given_input_with_outputs(
        input: &str,
        number_to_move: u32,
        from: usize,
        to: usize,
    ) {
        let output = parse_moves(input).unwrap();
        assert_eq!(number_to_move, output.1 .0);
        assert_eq!(from, output.1 .1);
        assert_eq!(to, output.1 .2);
    }
}

fn parse_crate_title(input: &str) -> nom::IResult<&str, Option<char>> {
    use nom::character::complete::anychar;
    use nom::sequence::tuple;

    let (rest, result) = tuple((anychar, anychar, anychar))(input)?;
    let result = if result.1.is_numeric() {
        Some(result.1)
    } else {
        None
    };
    Ok((rest, result))
}
fn parse_crate(input: &str) -> nom::IResult<&str, Option<char>> {
    use nom::character::complete::anychar;
    use nom::sequence::tuple;

    let (rest, result) = tuple((anychar, anychar, anychar))(input)?;
    let result = if result.1.is_alphabetic() {
        Some(result.1)
    } else {
        None
    };
    Ok((rest, result))
}

fn parse_crates(input: &str) -> nom::IResult<&str, Vec<Vec<char>>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha0, digit0, satisfy};
    use nom::combinator::value;
    use nom::multi::many0;
    use nom::multi::separated_list0;
    let mut retval = Vec::<Vec<char>>::new();

    let lines = input.lines().rev().collect::<Vec<&str>>();

    let (def_line, rest_lines) = lines.split_at(1);

    let (_, result) = separated_list0(tag(" "), parse_crate_title)(def_line[0])?;

    result.iter().for_each(|v| {
        if v.is_some() {
            retval.push(Vec::<char>::new());
        }
    });

    let mut rest = "";
    for l in rest_lines.iter() {
        let l = *l;
        let (rest_update, result) = separated_list0(tag(" "), parse_crate)(l)?;
        result.iter().enumerate().for_each(|(i, v)| {
            if let Some(c) = v {
                retval[i].push(*c);
            }
        });
        rest = rest_update;
    }

    Ok((rest, retval))
}
fn parse_moves(input: &str) -> nom::IResult<&str, (u32, usize, usize)> {
    use nom::bytes::complete::tag;
    use nom::character::complete::digit0;
    let (rest, _) = tag("move ")(input)?;
    let (rest, digit1) = digit0(rest)?;
    let (rest, _) = tag(" from ")(rest)?;
    let (rest, digit2) = digit0(rest)?;
    let (rest, _) = tag(" to ")(rest)?;
    let (rest, digit3) = digit0(rest)?;
    Ok((
        rest,
        (
            digit1.parse::<u32>().unwrap(),
            digit2.parse::<usize>().unwrap() - 1,
            digit3.parse::<usize>().unwrap() - 1,
        ),
    ))
}

fn move_between_stacks(stack: &mut Vec<Vec<char>>, number_to_move: u32, from: usize, to: usize) {
    for _ in 0..number_to_move {
        let c = stack[from].pop().unwrap();
        stack[to].push(c);
    }
}

fn main() -> std::io::Result<()> {
    //     const CRATES: &str = "    [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3 ";
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let (_, mut stacks) = parse_crates(split[0]).unwrap();
    split[1].lines().for_each(|l| {
        let (_, result) = parse_moves(l).unwrap();
        move_between_stacks(&mut stacks, result.0, result.1, result.2);
    });

    let mut output = String::with_capacity(stacks.len());
    for i in stacks.iter() {
        let c = i[i.len() - 1];
        // println!("{:?}", c);
        output.push(c);
    }
    // println!("{:?}", stacks);
    println!("{}", output);
    Ok(())
}
