struct Monkey {
    id: usize,
    divisible_by: u128,
    iftrue: usize,
    iffalse: usize,
    worry_calculation: fn(old: u128, argument: u128) -> u128,
    worry_argument: WorryArgument,
    items: Vec<u128>,
    inspected: u64,
}

enum WorryArgument {
    Old,
    Constant(u128),
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT1: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

    const TEST_INPUT2: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn parse_monkey_test() {
        let (_, monkey) = parse_monkey(TEST_INPUT1).unwrap();
        assert_eq!(0, monkey.id);
    }

    #[test]
    fn parse_monkeys_test() {
        let (_, monkeys) = parse_monkeys(TEST_INPUT2).unwrap();
    }

    #[test]
    fn calculate_worry_test() {
        let (_, mut monkeys) = parse_monkeys(TEST_INPUT2).unwrap();

        calculate_worry_part1(&mut monkeys);
        calculate_worry_part1(&mut monkeys);
        assert_eq!(0, monkeys[2].items.len());
        assert_eq!(0, monkeys[3].items.len());

        dbg!(monkeys[0].items.clone());
        assert_eq!(
            true,
            monkeys[0]
                .items
                .clone()
                .into_iter()
                .find(|u| *u == 695u128)
                .is_some()
        );
        assert_eq!(
            true,
            monkeys[0]
                .items
                .clone()
                .into_iter()
                .find(|u| *u == 10u128)
                .is_some()
        );
    }
    #[test]
    fn calculate_monkey_business_twenty_test() {
        let (_, mut monkeys) = parse_monkeys(TEST_INPUT2).unwrap();

        for _ in 0..20 {
            calculate_worry_part1(&mut monkeys);
        }
        let mut ordered_inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
        ordered_inspections.sort_by(|a, b| b.cmp(a));
        assert_eq!(105, ordered_inspections[0]);
        assert_eq!(101, ordered_inspections[1]);
    }

    #[test]
    fn calculate_monkey_business_part2_test() {
        let (_, mut monkeys) = parse_monkeys(TEST_INPUT2).unwrap();

        for _ in 0..10000 {
            calculate_worry_part2(&mut monkeys);
        }
        let mut ordered_inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
        dbg!(ordered_inspections.clone());
        ordered_inspections.sort_by(|a, b| b.cmp(a));
        let monkey_business = ordered_inspections[0] * ordered_inspections[1];
        assert_eq!(2713310158, monkey_business);
    }
}

fn add(old: u128, argument: u128) -> u128 {
    old + argument
}
fn sub(old: u128, argument: u128) -> u128 {
    old - argument
}
fn mult(old: u128, argument: u128) -> u128 {
    old * argument
}
fn div(old: u128, argument: u128) -> u128 {
    old / argument
}

fn parse_monkey(input: &str) -> nom::IResult<&str, Monkey> {
    use nom::bytes::complete::tag;
    use nom::character::complete::alphanumeric0;
    use nom::character::complete::char;
    use nom::character::complete::digit0;
    use nom::character::complete::newline;
    use nom::character::complete::one_of;
    use nom::multi::separated_list0;
    use nom::sequence::preceded;
    use nom::sequence::tuple;

    let (input, id) = preceded(tag("Monkey "), digit0)(input)?;
    let (input, _) = preceded(char(':'), newline)(input)?;
    let (input, starting_items) = preceded(
        tag("  Starting items: "),
        separated_list0(tag(", "), digit0),
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, operation) = preceded(
        tag("  Operation: new = old "),
        tuple((one_of("+-*/"), preceded(char(' '), alphanumeric0))),
    )(input)?;
    let (input, _) = newline(input)?;
    let (input, thetest) = preceded(tag("  Test: divisible by "), digit0)(input)?;
    let (input, _) = newline(input)?;
    let (input, iftrue) = preceded(tag("    If true: throw to monkey "), digit0)(input)?;
    let (input, _) = newline(input)?;
    let (input, iffalse) = preceded(tag("    If false: throw to monkey "), digit0)(input)?;

    let worry_calculation = match operation.0 {
        '+' => add,
        '-' => sub,
        '*' => mult,
        '/' => div,
        _ => panic!(),
    };

    let worry_argument = match operation.1 {
        "old" => WorryArgument::Old,
        _ => WorryArgument::Constant(operation.1.parse::<u128>().unwrap()),
    };

    Ok((
        input,
        Monkey {
            id: id.parse::<usize>().unwrap(),
            divisible_by: thetest.parse::<u128>().unwrap(),
            iftrue: iftrue.parse::<usize>().unwrap(),
            iffalse: iffalse.parse::<usize>().unwrap(),
            worry_calculation,
            worry_argument,
            items: starting_items
                .iter()
                .map(|si| si.parse::<u128>().unwrap())
                .collect::<Vec<u128>>(),
            inspected: 0,
        },
    ))
}

fn parse_monkeys(input: &str) -> nom::IResult<&str, Vec<Monkey>> {
    use nom::character::complete::newline;
    use nom::multi::count;
    use nom::multi::separated_list0;

    let (input, monkeys) = separated_list0(count(newline, 2), parse_monkey)(input)?;
    Ok((input, monkeys))
}

fn calculate_worry_common(monkies: &mut Vec<Monkey>, reduce_worry_after_inspection: bool) {
    let mut common_denominator = monkies[0].divisible_by;
    for i in 1..monkies.len() {
        common_denominator *= monkies[i].divisible_by;
    }
    for i in 0..monkies.len() {
        let mut items = Vec::<u128>::new();
        std::mem::swap(&mut monkies[i].items, &mut items);
        monkies[i].inspected += items.len() as u64;
        for item in items {
            let calc = monkies[i].worry_calculation;
            let new_worry = match monkies[i].worry_argument {
                WorryArgument::Old => calc(item, item),
                WorryArgument::Constant(c) => calc(item, c),
            };

            let iftrue = monkies[i].iftrue;
            let iffalse = monkies[i].iffalse;

            let new_worry_after = if reduce_worry_after_inspection {
                (new_worry / 3) % common_denominator
            } else {
                new_worry % common_denominator
            };
            if new_worry_after % monkies[i].divisible_by == 0 {
                monkies[iftrue].items.push(new_worry_after);
            } else {
                monkies[iffalse].items.push(new_worry_after);
            }
        }
    }
}

fn calculate_worry_part1(monkies: &mut Vec<Monkey>) {
    calculate_worry_common(monkies, true);
}
fn calculate_worry_part2(monkies: &mut Vec<Monkey>) {
    calculate_worry_common(monkies, false);
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let (_, mut monkeys) = parse_monkeys(&input).unwrap();

    for _ in 0..20 {
        calculate_worry_part1(&mut monkeys);
    }
    let mut ordered_inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    ordered_inspections.sort_by(|a, b| b.cmp(a));
    println!("{}", ordered_inspections[0] * ordered_inspections[1]);

    let (_, mut monkeys) = parse_monkeys(&input).unwrap();

    for _ in 0..10000 {
        calculate_worry_part2(&mut monkeys);
    }
    let mut ordered_inspections = monkeys.iter().map(|m| m.inspected).collect::<Vec<u64>>();
    ordered_inspections.sort_by(|a, b| b.cmp(a));
    println!("{}", ordered_inspections[0] * ordered_inspections[1]);
}
