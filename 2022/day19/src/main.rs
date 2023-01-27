use std::{collections::HashMap, thread::JoinHandle};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_maximum_ore_production_test() {
        let (_, designs) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(2, designs.len());

        assert_eq!(9, find_max_geode(24, &designs[0], &Resources::new()));
    }
}

const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

#[derive(Debug)]
struct Cost {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

#[derive(Debug)]
struct Design {
    _id: u8,
    ore: Cost,
    clay: Cost,
    obsidian: Cost,
    geode: Cost,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: u8,
    geode_robot: u8,
}

enum Action {
    NoAction,
    MakeOreRobot,
    MakeClayRobot,
    MakeObsidianRobot,
    MakeGeodeRobot,
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

impl Cost {
    fn can_make_robot(&self, resources: &Resources) -> bool {
        resources.ore >= self.ore
            && resources.clay >= self.clay
            && resources.obsidian >= self.obsidian
    }
}

impl Resources {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0,
        }
    }
    fn process_resources(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }
    fn make_robot(&mut self, cost: &Cost) {
        self.ore -= cost.ore;
        self.clay -= cost.clay;
        self.obsidian -= cost.obsidian;
    }
}

fn find_max_geode(minutes: u32, design: &Design, robots_resources: &Resources) -> u32 {
    let mut cache = HashMap::<(u32, Resources), u32>::new();

    find_max_geode_step(&mut cache, minutes, design, &robots_resources)
}

fn find_max_geode_step(
    cache: &mut HashMap<(u32, Resources), u32>,
    minutes: u32,
    design: &Design,
    resources: &Resources,
) -> u32 {
    if minutes == 0 {
        return 0;
    }
    // if cache.
    let mut options = Vec::<Action>::with_capacity(5);
    options.push(Action::NoAction);
    if design.ore.can_make_robot(&resources) {
        options.push(Action::MakeOreRobot);
    }
    if design.clay.can_make_robot(&resources) {
        options.push(Action::MakeClayRobot);
    }
    if design.obsidian.can_make_robot(&resources) {
        options.push(Action::MakeObsidianRobot);
    }
    if design.geode.can_make_robot(&resources) {
        options.push(Action::MakeGeodeRobot);
    }
    let mut max_geodes_processed: u32 = 0;
    for i in options {
        let mut resources = resources.clone();
        let geodes_processed = match i {
            Action::NoAction => {
                resources.process_resources();
                find_max_geode_step(cache, minutes - 1, design, &resources)
            }
            Action::MakeOreRobot => {
                resources.make_robot(&design.ore);
                resources.process_resources();
                resources.ore_robot += 1;
                find_max_geode_step(cache, minutes - 1, design, &resources)
            }
            Action::MakeClayRobot => {
                resources.make_robot(&design.clay);
                resources.process_resources();
                resources.clay_robot += 1;
                find_max_geode_step(cache, minutes - 1, design, &resources)
            }
            Action::MakeObsidianRobot => {
                resources.make_robot(&design.obsidian);
                resources.process_resources();
                resources.obsidian_robot += 1;
                find_max_geode_step(cache, minutes - 1, design, &resources)
            }
            Action::MakeGeodeRobot => {
                resources.make_robot(&design.geode);
                resources.process_resources();
                resources.geode_robot += 1;
                find_max_geode_step(cache, minutes - 1, design, &resources)
            }
            _ => panic!(),
        };
        max_geodes_processed = geodes_processed.max(max_geodes_processed) + resources.geode as u32;
    }
    cache.insert((minutes, resources.clone()), max_geodes_processed);
    // println!(
    // "{} {} {} {}",
    // resources.ore, resources.clay, resources.obsidian, resources.geode
    // );
    max_geodes_processed
}

fn parse_input_line<'a>(input: &'a str) -> nom::IResult<&'a str, Design> {
    use nom::bytes::complete::tag;
    use nom::character::complete::u8;
    use nom::sequence::delimited;
    use nom::sequence::preceded;
    use nom::sequence::tuple;

    let (remaining, t) = tuple((
        preceded(tag("Blueprint "), u8),
        preceded(tag(": Each ore robot costs "), u8),
        preceded(tag(" ore. Each clay robot costs "), u8),
        preceded(tag(" ore. Each obsidian robot costs "), u8),
        preceded(tag(" ore and "), u8),
        preceded(tag(" clay. Each geode robot costs "), u8),
        delimited(tag(" ore and "), u8, tag(" obsidian.")),
    ))(input)?;
    let design = Design {
        _id: t.0,
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
    // let (_, designs) = parse_input(&input).unwrap();
    let (_, designs) = parse_input(TEST_INPUT).unwrap();
    let mut thandles = Vec::<JoinHandle<u32>>::new();
    for d in designs.into_iter() {
        thandles.push(std::thread::spawn(move || {
            find_max_geode(24, &d, &Resources::new())
        }));
    }

    let mut max_value_found: u32 = 0;

    for i in thandles {
        let j = i.join();
        max_value_found = max_value_found.max(j.unwrap());
    }
    println!("{}", max_value_found);
}
