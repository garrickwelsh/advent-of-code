use rayon::prelude::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_maximum_ore_production_test() {
        let (_, designs) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(2, designs.len());

        assert_eq!(9, find_max_geode_score(24, &designs[0], &Resources::new()));
    }

    #[test]
    fn calculate_maximum_ore_production_both_designs_test() {
        let (_, designs) = parse_input(TEST_INPUT).unwrap();
        assert_eq!(
            33u32,
            designs
                .par_iter()
                .map(|d| find_max_geode_score(24, d, &Resources::new()))
                .sum()
        );
    }
}

#[cfg(test)]
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

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: u32,
    geode_robot: u32,
}

#[derive(Debug, Clone, Copy)]
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

fn find_max_geode_score(minutes: u32, design: &Design, robots_resources: &Resources) -> u32 {
    let retval = find_max_geode_steps(minutes, design, &robots_resources);
    let r = retval * design.id;
    // println!("{}", r);
    r
}

fn find_max_geodes(minutes: u32, design: &Design, robots_resources: &Resources) -> u32 {
    let retval = find_max_geode_steps(minutes, design, &robots_resources);
    retval
}

fn find_max_geode_steps(minutes: u32, design: &Design, resource: &Resources) -> u32 {
    let mut resources = Vec::<Resources>::new();
    resources.push(resource.clone());

    let mut options = Vec::<Action>::with_capacity(5);
    for _ in (0..minutes).rev() {
        let mut next_minute = Vec::<Resources>::new();

        let max_geode_robots = resources.iter().map(|r| r.geode_robot).max().unwrap();
        let max_obsidian_robots = resources.iter().map(|r| r.obsidian_robot).max().unwrap();
        let max_geodes = resources.iter().map(|r| r.geode).max().unwrap();

        for resource in resources.into_iter().filter(|r| {
            r.geode_robot == max_geode_robots
                || r.obsidian_robot == max_obsidian_robots
                || r.geode == max_geodes
        }) {
            if design.geode.can_make_robot(&resource) {
                options.push(Action::MakeGeodeRobot);
            }
            if design.obsidian.can_make_robot(&resource)
                && design.geode.obsidian > resource.obsidian_robot
            {
                options.push(Action::MakeObsidianRobot);
            }
            if design.clay.can_make_robot(&resource) && design.obsidian.clay > resource.clay_robot {
                options.push(Action::MakeClayRobot);
            }
            if design.ore.can_make_robot(&resource) && design.clay.ore > resource.ore_robot {
                options.push(Action::MakeOreRobot);
            }
            // Once we reach the max available ore we can no longer do nothing.
            // if resource.ore < ore_limit * 2 {
            options.push(Action::NoAction);
            // }

            for i in options.iter() {
                let mut r = resource.clone();
                match i {
                    Action::MakeGeodeRobot => {
                        r.make_robot(&design.geode);
                        r.process_resources();
                        r.geode_robot += 1;
                    }
                    Action::MakeObsidianRobot => {
                        r.make_robot(&design.obsidian);
                        r.process_resources();
                        r.obsidian_robot += 1;
                    }
                    Action::MakeClayRobot => {
                        r.make_robot(&design.clay);
                        r.process_resources();
                        r.clay_robot += 1;
                    }
                    Action::MakeOreRobot => {
                        r.make_robot(&design.ore);
                        r.process_resources();
                        r.ore_robot += 1;
                    }
                    Action::NoAction => {
                        r.process_resources();
                    }
                };
                next_minute.push(r);
            }
            options.clear();
        }
        resources = next_minute;
    }
    resources.iter().map(|r| r.geode).max().unwrap()
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
    let (_, designs) = parse_input(&trimmed_input).unwrap();
    // Part 1
    let result: u32 = designs
        .par_iter()
        .map(|d| find_max_geode_score(24, d, &Resources::new()))
        .sum();
    println!("{}", result);

    // Part 2
    let result: u32 = designs[0..=2]
        .par_iter()
        .map(|d| find_max_geodes(32, d, &Resources::new()))
        .product();
    println!("{}", result);
}
