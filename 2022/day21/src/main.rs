#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn parse_input_test() {
        let (remaining, monkeys) = parse_input(TEST_INPUT).unwrap();
        assert_eq!("", remaining);
        assert_eq!("root", monkeys[0].name);
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

enum MonkeyValue<'a> {
    Constant(i32),
    Name(&'a str),
}

enum MonkeyBusiness<'a> {
    Constant(MonkeyValue<'a>),
    Add(MonkeyValue<'a>, MonkeyValue<'a>),
    Subtract(MonkeyValue<'a>, MonkeyValue<'a>),
    Multiply(MonkeyValue<'a>, MonkeyValue<'a>),
    Divide(MonkeyValue<'a>, MonkeyValue<'a>),
}

struct Monkey<'a> {
    name: &'a str,
    monkey_business: MonkeyBusiness<'a>,
}

fn parse_value<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyValue> {
    use nom::branch::alt;
    use nom::character::complete::alpha1;
    use nom::character::complete::i32;
    use nom::combinator::map;

    alt((
        map(alpha1, |a| MonkeyValue::Name(a)),
        map(i32, |a| MonkeyValue::Constant(a)),
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
    Ok((remaining, MonkeyBusiness::Add(t.0, t.1)))
}
fn parse_multiply<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyBusiness> {
    use nom::bytes::complete::tag;
    use nom::sequence::separated_pair;

    let (remaining, t) = separated_pair(parse_value, tag(" * "), parse_value)(input)?;
    Ok((remaining, MonkeyBusiness::Add(t.0, t.1)))
}
fn parse_divide<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyBusiness> {
    use nom::bytes::complete::tag;
    use nom::sequence::separated_pair;

    let (remaining, t) = separated_pair(parse_value, tag(" / "), parse_value)(input)?;
    Ok((remaining, MonkeyBusiness::Add(t.0, t.1)))
}
fn parse_constant<'a>(input: &'a str) -> nom::IResult<&'a str, MonkeyBusiness> {
    use nom::character::complete::i32;
    use nom::combinator::map;

    map(i32, |c| MonkeyBusiness::Constant(MonkeyValue::Constant(c)))(input)
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

fn main() {
    println!("Hello, world!");
}
