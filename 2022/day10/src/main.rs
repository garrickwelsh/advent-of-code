const UNIT_TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_line_test() {
        const LINE1: &str = "addx -32";
        let (_, result) = parse_line(LINE1).unwrap();
        assert_eq!(Action::AddX(-32), result);
    }

    #[test]
    fn run_operations_test() {
        let actions = parse_lines(UNIT_TEST_INPUT).unwrap().1;
        let register_log = run_actions_for_part1(&actions);
        assert_eq!(420, register_log[0].strength);
        assert_eq!(1140, register_log[1].strength);
        assert_eq!(1800, register_log[2].strength);
        assert_eq!(2940, register_log[3].strength);
        assert_eq!(2880, register_log[4].strength);
        assert_eq!(3960, register_log[5].strength);

        assert_eq!(13140, register_log.iter().map(|r| r.strength).sum::<i32>());
    }
}

#[derive(Debug, PartialEq)]
enum Action {
    NoOp,
    AddX(i32),
}

struct Signal {
    register: i32,
    cycle: i32,
    strength: i32,
}

fn parse_line(input: &str) -> nom::IResult<&str, Action> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::char;
    use nom::character::complete::not_line_ending;
    use nom::sequence::preceded;

    let (mut remaining, output) = alt((tag("addx"), tag("noop")))(input)?;
    let mut action = Action::NoOp;
    if output == "addx" {
        let (remains, number) = preceded(char(' '), not_line_ending)(remaining)?;

        action = Action::AddX(number.parse::<i32>().unwrap());
        remaining = remains;
    }

    Ok((remaining, action))
}

fn parse_lines(input: &str) -> nom::IResult<&str, Vec<Action>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;
    let (remaining, result) = separated_list0(newline, parse_line)(input)?;

    Ok((remaining, result))
}

fn run_actions_for_part1(actions: &Vec<Action>) -> Vec<Signal> {
    run_actions(actions)
        .into_iter()
        .enumerate()
        .filter(|(i, _)| {
            let i = *i + 1;
            (i + 20) % 40 == 0
        })
        .map(|(_, v)| v)
        .collect::<Vec<Signal>>()
}

fn run_actions(actions: &Vec<Action>) -> Vec<Signal> {
    const NO_OP: Action = Action::NoOp;

    let mut results = Vec::<Signal>::new();

    let mut cycle = 1i32;
    let mut register = 1i32;
    let mut actioncountdown = 0;

    let mut action: &Action = &NO_OP;
    let mut iterator = actions.iter();
    loop {
        if actioncountdown == 0 {
            if let Action::AddX(a) = action {
                register += *a;
            }
        }
        results.push(Signal {
            cycle,
            register,
            strength: register * cycle,
        });
        if actioncountdown == 0 {
            let possible_action = iterator.next();
            if possible_action.is_none() {
                break;
            }
            action = possible_action.unwrap();
            (cycle, register) = match action {
                Action::NoOp => (cycle, register),
                Action::AddX(_) => {
                    actioncountdown = 1;
                    (cycle, register)
                }
            }
        } else {
            actioncountdown -= 1;
        }
        cycle += 1;
    }

    results
}

fn screen_for_part2(signals: &Vec<Signal>) -> String {
    let mut screen = String::with_capacity(250);

    for (i, v) in signals.iter().enumerate() {
        let mut to_draw = false;
        let i = i as i32;
        if i != 0 && i % 40 == 0 {
            screen.push_str("\n");
        }
        let i = i % 40;
        println!("{} - {}", i + 1, v.register);
        for j in i - 1..=i + 1 {
            if j as i32 == v.register {
                to_draw = true;
            }
        }
        if to_draw {
            screen.push('#');
        } else {
            screen.push('.');
        }
    }

    screen
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let actions = parse_lines(&input).unwrap().1;
    let results = run_actions_for_part1(&actions);
    println!("{}", results.iter().map(|m| m.strength).sum::<i32>());
    // let actions = parse_lines(UNIT_TEST_INPUT).unwrap().1;
    let results = run_actions(&actions);
    // let screen = screen_for_part2(&results);
    let screen = screen_for_part2(&results);
    println!("{}", screen);
}
