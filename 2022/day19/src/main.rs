#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_maximum_ore_production_test() {
        let (_, designs) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(2, designs.len());
    }
}

const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

#[derive(Debug)]
struct Cost {
    ore: u32,
    clay: u32,
    obsidian: u32,
}

#[derive(Debug)]
struct Design {
    id: u32,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

impl Default for Cost {
    fn default() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
        }
    }
}

fn parse_input_line<'a>(input: &'a str) -> nom::IResult<&'a str, Design> {
    use nom::bytes::complete::tag;
    use nom::character::complete::u32;
    use nom::sequence::delimited;
    use nom::sequence::preceded;
    use nom::sequence::tuple;

    let (remaining, t) = tuple((
        preceded(tag("Blueprint "), u32),
        preceded(tag(": Each ore robot costs "), u32),
        preceded(tag(" ore. Each clay robot costs "), u32),
        preceded(tag(" ore. Each obsidian robot costs "), u32),
        preceded(tag(" ore and "), u32),
        preceded(tag(" clay. Each geode robot costs "), u32),
        delimited(tag(" ore and "), u32, tag(" obsidian.")),
    ))(input)?;
    let design = Design {
        id: t.0,
        ore: Cost {
            ore: t.1,
            clay: 0,
            obsidian: 0,
        },
        clay: Cost {
            ore: t.2,
            clay: 0,
            obsidian: 0,
        },
        obsidian: Cost {
            ore: t.3,
            clay: t.4,
            obsidian: 0,
        },
        geode: Cost {
            ore: t.5,
            clay: 0,
            obsidian: t.6,
        },
    };
    Ok((remaining, design))
}

fn parse_input<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<Design>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;

    separated_list0(newline, parse_input_line)(input)
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
    let (_, designs) = parse_input(&input).unwrap();
    dbg!(designs);
    todo!();
}
