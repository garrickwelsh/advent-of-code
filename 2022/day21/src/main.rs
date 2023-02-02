use std::{collections::HashMap, fmt::Display, };

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_input_test() {
        let (remaining, monkeys) = parse_input(TEST_INPUT).unwrap();
        assert_eq!("", remaining);
        assert_eq!("root", monkeys[0].name);
    }

    #[test]
    fn make_monkey_maths() {
        let (_, monkeys) = parse_input(TEST_INPUT).unwrap();
        let mut mm = MonkeyMaths::new(monkeys);
        println!("Solved");
        for m in mm.solved.iter() {
            println!("{}", m.1);
            println!("{}", m.1.calculate_value().unwrap());
        }
        println!("Unresolved");
        for m in mm.bhs.values() {
            println!("{}", m);
        }
        mm.resolve_monkey_business();
        println!("After resolve");
        for m in mm.solved.iter() {
            println!("{}", m.1);
            println!("{}", m.1.calculate_value().unwrap());
        }
        println!("Unresolved");
        for m in mm.bhs.values() {
            println!("{}", m);
        }
        assert_eq!(
            Some(152),
            mm.solved.get("root").expect("root").calculate_value()
        );
    }
}
const TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

#[derive(Copy, Clone, Debug)]
enum MonkeyValue<'a> {
    Constant(i64),
    Name(&'a str),
}

#[derive(Debug)]
enum MonkeyBusiness<'a> {
    Constant(MonkeyValue<'a>),
    Add(MonkeyValue<'a>, MonkeyValue<'a>),
    Subtract(MonkeyValue<'a>, MonkeyValue<'a>),
    Multiply(MonkeyValue<'a>, MonkeyValue<'a>),
    Divide(MonkeyValue<'a>, MonkeyValue<'a>),
}

#[derive(Debug)]
struct Monkey<'a> {
    name: &'a str,
    monkey_business: MonkeyBusiness<'a>,
}

impl<'a> Display for Monkey<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name, self.monkey_business)
    }
}
impl<'a> Display for MonkeyBusiness<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonkeyBusiness::Constant(c) => write!(f, "{}", c),
            MonkeyBusiness::Add(l, r) => write!(f, "{} + {}", l, r),
            MonkeyBusiness::Subtract(l, r) => write!(f, "{} - {}", l, r),
            MonkeyBusiness::Multiply(l, r) => write!(f, "{} * {}", l, r),
            MonkeyBusiness::Divide(l, r) => write!(f, "{} / {}", l, r),
        }
    }
}
impl<'a> Display for MonkeyValue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MonkeyValue::Constant(c) => write!(f, "{}", c),
            MonkeyValue::Name(n) => write!(f, "{}", n),
        }
    }
}

impl<'a> MonkeyValue<'a> {
    fn calculate_value(&self) -> Option<i64> {
        match self {
            MonkeyValue::Constant(x) => Some(*x),
            _ => None,
        }
    }

    fn replace_name_with_value(&self, name: &'a str, value: i64) -> Self {
        match self {
            MonkeyValue::Name(n) => {
                if *n == name {
                    MonkeyValue::Constant(value)
                } else {
                    *self
                }
            }
            _ => *self,
        }
    }
}

impl<'a> Monkey<'a> {
    fn calculate_value(&self) -> Option<i64> {
        match self.monkey_business {
            MonkeyBusiness::Constant(c) => c.calculate_value(),
            MonkeyBusiness::Add(lhs, rhs) => {
                let lhs = lhs.calculate_value();
                let rhs = rhs.calculate_value();
                if lhs.is_some() && rhs.is_some() {
                    Some(lhs.unwrap() + rhs.unwrap())
                } else {
                    None
                }
            }
            MonkeyBusiness::Multiply(lhs, rhs) => {
                let lhs = lhs.calculate_value();
                let rhs = rhs.calculate_value();
                if lhs.is_some() && rhs.is_some() {
                    Some(lhs.unwrap() * rhs.unwrap())
                } else {
                    None
                }
            }
            MonkeyBusiness::Subtract(lhs, rhs) => {
                let lhs = lhs.calculate_value();
                let rhs = rhs.calculate_value();
                if lhs.is_some() && rhs.is_some() {
                    Some(lhs.unwrap() - rhs.unwrap())
                } else {
                    None
                }
            }
            MonkeyBusiness::Divide(lhs, rhs) => {
                let lhs = lhs.calculate_value();
                let rhs = rhs.calculate_value();
                if lhs.is_some() && rhs.is_some() {
                    Some(lhs.unwrap() / rhs.unwrap())
                } else {
                    None
                }
            }
        }
    }

    fn replace_name_with_value(&mut self, name: &'a str, calculated_value: i64) {
        self.monkey_business = match self.monkey_business {
            MonkeyBusiness::Add(lhs, rhs) => MonkeyBusiness::Add(
                lhs.replace_name_with_value(name, calculated_value),
                rhs.replace_name_with_value(name, calculated_value),
            ),
            MonkeyBusiness::Multiply(lhs, rhs) => MonkeyBusiness::Multiply(
                lhs.replace_name_with_value(name, calculated_value),
                rhs.replace_name_with_value(name, calculated_value),
            ),
            MonkeyBusiness::Subtract(lhs, rhs) => MonkeyBusiness::Subtract(
                lhs.replace_name_with_value(name, calculated_value),
                rhs.replace_name_with_value(name, calculated_value),
            ),
            MonkeyBusiness::Divide(lhs, rhs) => MonkeyBusiness::Divide(
                lhs.replace_name_with_value(name, calculated_value),
                rhs.replace_name_with_value(name, calculated_value),
            ),
            MonkeyBusiness::Constant(c) => MonkeyBusiness::Constant(c),
        }
    }
}

fn parse_value<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyValue> {
    use nom::branch::alt;
    use nom::character::complete::alpha1;
    use nom::character::complete::i64;
    use nom::combinator::map;

    alt((
        map(alpha1, |a| MonkeyValue::Name(a)),
        map(i64, |a| MonkeyValue::Constant(a)),
    ))(input)
}

fn parse_add<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyBusiness> {
    use nom::bytes::complete::tag;
    use nom::sequence::separated_pair;

    let (remaining, t) = separated_pair(parse_value, tag(" + "), parse_value)(input)?;
    Ok((remaining, MonkeyBusiness::Add(t.0, t.1)))
}

fn parse_subtract<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyBusiness> {
    use nom::bytes::complete::tag;
    use nom::sequence::separated_pair;

    let (remaining, t) = separated_pair(parse_value, tag(" - "), parse_value)(input)?;
    Ok((remaining, MonkeyBusiness::Subtract(t.0, t.1)))
}
fn parse_multiply<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyBusiness> {
    use nom::bytes::complete::tag;
    use nom::sequence::separated_pair;

    let (remaining, t) = separated_pair(parse_value, tag(" * "), parse_value)(input)?;
    Ok((remaining, MonkeyBusiness::Multiply(t.0, t.1)))
}
fn parse_divide<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyBusiness> {
    use nom::bytes::complete::tag;
    use nom::sequence::separated_pair;

    let (remaining, t) = separated_pair(parse_value, tag(" / "), parse_value)(input)?;
    Ok((remaining, MonkeyBusiness::Divide(t.0, t.1)))
}
fn parse_constant<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyBusiness> {
    use nom::character::complete::i64;
    use nom::combinator::map;

    map(i64, |c| MonkeyBusiness::Constant(MonkeyValue::Constant(c)))(input)
}
fn parse_monkey<'a>(input: &'a str) -> nom::IResult<&'a str, Monkey> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::alpha1;
    use nom::sequence::terminated;

    let (remaining, name) = terminated(alpha1, tag(": "))(input)?;
    let (remaining, monkey_business) = alt((
        parse_add,
        parse_subtract,
        parse_multiply,
        parse_divide,
        parse_constant,
    ))(remaining)?;
    let monkey = Monkey {
        name,
        monkey_business,
    };
    Ok((remaining, monkey))
}

fn parse_input<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<Monkey>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;

    separated_list0(newline, parse_monkey)(input)
}

#[derive(Debug)]
struct MonkeyMaths<'a> {
    solved: HashMap<&'a str, Monkey<'a>>,
    bhs: HashMap<(Option<&'a str>, Option<&'a str>), Monkey<'a>>,
}

impl<'a> MonkeyMaths<'a> {
    fn new(monkeys: Vec<Monkey<'a>>) -> Self {
        let mut monkey_maths = Self {
            solved: HashMap::<&'a str, Monkey<'a>>::new(),
            bhs: HashMap::<(Option<&'a str>, Option<&'a str>), Monkey<'a>>::new(),
        };
        for i in monkeys.into_iter() {
            match i.monkey_business {
                MonkeyBusiness::Constant(_) => _ = monkey_maths.solved.insert(i.name, i),
                MonkeyBusiness::Add(lhs, rhs) => monkey_maths.add_monkey_values(i, &lhs, &rhs),
                MonkeyBusiness::Multiply(lhs, rhs) => monkey_maths.add_monkey_values(i, &lhs, &rhs),
                MonkeyBusiness::Subtract(lhs, rhs) => monkey_maths.add_monkey_values(i, &lhs, &rhs),
                MonkeyBusiness::Divide(lhs, rhs) => monkey_maths.add_monkey_values(i, &lhs, &rhs),
            }
        }
        monkey_maths
    }

    fn solve_for_name(&mut self, monkey: &mut Monkey<'a>, key: Option<&'a str>) -> bool {
        let mut solved = false;
        let Some(key) = key else {return solved;};
        if self.solved.contains_key(key) {
            let val = self.solved.get(key).unwrap().calculate_value().unwrap();
            monkey.replace_name_with_value(key, val);
            solved = true;
        }
        solved
    }

    fn resolve_monkey_business(&mut self) {
        while self.bhs.iter().count() > 0 {
            let mut bhs = HashMap::<(Option<&'a str>, Option<&'a str>), Monkey>::new();
            std::mem::swap(&mut bhs, &mut self.bhs);
            for (key, mut monkey) in bhs.into_iter() {
                let mut k1 = key.0;
                let mut k2 = key.1;
                if self.solve_for_name(&mut monkey, k1) {
                    k1 = None;
                }
                if self.solve_for_name(&mut monkey, k2) {
                    k2 = None;
                }
                if monkey.calculate_value().is_some() {
                    self.solved.insert(monkey.name, monkey);
                } else {
                    self.bhs.insert((k1, k2), monkey);
                }
            }
        }
    }

    fn add_monkey_values(
        &mut self,
        monkey: Monkey<'a>,
        lhs: &MonkeyValue<'a>,
        rhs: &MonkeyValue<'a>,
    ) {
        let mut k1: Option<&'a str> = None;
        let mut k2: Option<&'a str> = None;
        match lhs {
            MonkeyValue::Name(name) => {
                k1 = Some(name);
            }
            _ => panic!(),
        };
        match rhs {
            MonkeyValue::Name(name) => {
                k2 = Some(name);
            }
            _ => panic!(),
        };
        self.bhs.insert((k1, k2), monkey);
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

    let trimmed_input = input.trim();
    let (_, monkeys) = parse_input(trimmed_input).unwrap();
    let mut mm = MonkeyMaths::new(monkeys);
    mm.resolve_monkey_business();
    println!("root - {}", mm.solved.get("root").unwrap());
    println!(
        "root value - {}",
        mm.solved.get("root").unwrap().calculate_value().unwrap()
    );
}
