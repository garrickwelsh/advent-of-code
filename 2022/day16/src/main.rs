use std::collections::HashMap;

const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parser_test() {
        let (remaining, valves) = parse_input(TEST_INPUT).unwrap();
        assert_eq!("", remaining);
        assert_eq!("AA", valves[0].name);
    }

    #[test]
    fn maximise_test() {
        let (_remaining, valves) = parse_input(TEST_INPUT).unwrap();
        let valve_map = calculate_initial_map(valves);
        let move_map = calculate_map(&valve_map);
        let mut closed_valves = get_closed_valves(&valve_map);
        assert_eq!(
            1651,
            calculate_maximum_releasable_pressure(
                &valve_map,
                &move_map,
                &mut closed_valves,
                30,
                "AA"
            )
            .0
        );
    }
    #[test]
    fn maximise_part2_test() {
        let (_remaining, valves) = parse_input(TEST_INPUT).unwrap();
        let valve_map = calculate_initial_map(valves);
        let move_map = calculate_map(&valve_map);
        let closed_valves = get_closed_valves(&valve_map);
        assert_eq!(
            1707,
            p2_calculate_maximum_releasable_pressure(
                &valve_map,
                &move_map,
                &closed_valves,
                26,
                "AA",
                "AA"
            )
        );
    }
}

struct Valve<'a> {
    valve_id: u32, // will be an value that can be used as a bit field for open valves
    leads_to: Vec<&'a str>,
    name: &'a str,
    pressure_released: i32,
}

fn parse_valve<'a>(input: &'a str) -> nom::IResult<&str, Valve<'a>> {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::alpha1;
    use nom::character::complete::i32;
    use nom::multi::separated_list0;
    use nom::sequence::preceded;
    use nom::sequence::tuple;

    // Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    let (remaining, t) = tuple((
        preceded(tag("Valve "), alpha1),
        preceded(tag(" has flow rate="), i32),
        alt((
            preceded(
                tag("; tunnels lead to valves "),
                separated_list0(tag(", "), alpha1),
            ),
            preceded(
                tag("; tunnel leads to valve "),
                separated_list0(tag(", "), alpha1),
            ),
        )),
    ))(input)?;
    let valve = Valve {
        valve_id: 0,
        name: t.0,
        pressure_released: t.1,
        leads_to: t.2,
        // is_open: false,
    };
    Ok((remaining, valve))
}

fn parse_input_internal<'a>(input: &'a str) -> nom::IResult<&str, Vec<Valve<'a>>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;
    separated_list0(newline, parse_valve)(input)
}
fn parse_input<'a>(input: &'a str) -> nom::IResult<&str, Vec<Valve<'a>>> {
    let (remaining, mut valves) = parse_input_internal(input)?;
    let mut i = 0;
    for j in 0..valves.len() {
        if valves[j].pressure_released > 0 {
            valves[j].valve_id = 1 << i;
            i += 1;
        }
    }

    Ok((remaining, valves))
}

fn calculate_initial_map<'a>(valves: Vec<Valve<'a>>) -> HashMap<&'a str, Valve<'a>> {
    let mut map = HashMap::<&'a str, Valve<'a>>::new();
    for i in valves.into_iter() {
        let current_valve_name: &'a str = i.name;
        map.insert(current_valve_name, i);
    }
    map
}
fn calculate_map<'a>(map: &HashMap<&'a str, Valve<'a>>) -> HashMap<(&'a str, &'a str), i32> {
    let keys = map.keys().map(|k| *k).collect::<Vec<&'a str>>();

    let mut retval = HashMap::<(&'a str, &'a str), i32>::new();

    for i in 0..keys.len() {
        let i = keys[i];
        for j in 0..keys.len() {
            let j = keys[j];
            if i == j {
                continue;
            }

            // Breadth first search
            let mut depth = 1;
            let mut to_search = vec![i];
            loop {
                if to_search.contains(&j) {
                    retval.insert((i, j), depth);
                    break;
                }
                let mut to_search_next = Vec::<&'a str>::new();
                for l in to_search {
                    map[l]
                        .leads_to
                        .iter()
                        .for_each(|lt| to_search_next.push(*lt));
                }
                to_search = to_search_next;
                depth += 1;
            }
        }
    }
    retval
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Progress<'a> {
    current_valve: &'a str,
    time_remaining: i32,
}

fn get_closed_valves_as_u32<'a>(
    valve_map: &HashMap<&'a str, Valve<'a>>,
    closed_valves: &Vec<&'a str>,
) -> u32 {
    let mut closed_valves_u32 = 0;
    for i in closed_valves {
        closed_valves_u32 |= valve_map[i].valve_id;
    }
    closed_valves_u32
}

fn p2_calculate_maximum_releasable_pressure_step<'a>(
    valve_map: &HashMap<&'a str, Valve<'a>>,
    move_map: &HashMap<(&'a str, &'a str), i32>,
    closed_valves: &Vec<&'a str>,
    cache: &mut HashMap<(Progress<'a>, Progress<'a>, u32), i32>,
    p1: &Progress<'a>,
    p2: &Progress<'a>,
    route: &mut Vec<&'a str>,
) -> i32 {
    let (pmodified, psame) = if p1.time_remaining >= p2.time_remaining {
        (p1, p2)
    } else {
        (p2, p1)
    };
    if pmodified.time_remaining < 0 {
        return 0;
    }
    route.push(pmodified.current_valve);

    let mut max_flow = 0;

    let closed_valves_u32 = get_closed_valves_as_u32(valve_map, closed_valves);

    let key = (pmodified.clone(), psame.clone(), closed_valves_u32);
    if cache.contains_key(&key) {
        return cache[&key];
    }

    if closed_valves.len() == 0 {
        max_flow += pmodified.time_remaining * valve_map[pmodified.current_valve].pressure_released;
        max_flow += psame.time_remaining * valve_map[psame.current_valve].pressure_released;
        cache.insert(key, max_flow);
        return max_flow;
    }

    for i in 0..closed_valves.len() {
        let next_valve = closed_valves[i];
        let mut closed_valves = closed_valves.clone();
        closed_valves.swap_remove(i);

        let distance = move_map[&(pmodified.current_valve, next_valve)];

        let pn = Progress {
            current_valve: next_valve,
            time_remaining: pmodified.time_remaining - distance,
        };
        let flow = p2_calculate_maximum_releasable_pressure_step(
            valve_map,
            move_map,
            &closed_valves,
            cache,
            &pn,
            psame,
            route,
        );
        max_flow = flow.max(max_flow);
    }

    route.pop();
    max_flow += pmodified.time_remaining * valve_map[pmodified.current_valve].pressure_released;
    cache.insert(key, max_flow);
    max_flow
}
fn p2_calculate_maximum_releasable_pressure<'a>(
    valve_map: &HashMap<&'a str, Valve<'a>>,
    move_map: &HashMap<(&'a str, &'a str), i32>,
    closed_valves: &Vec<&'a str>,
    time_remaining: i32,
    p1_start: &'static str,
    p2_start: &'static str,
) -> i32 {
    let p1 = Progress {
        current_valve: p1_start,
        time_remaining,
    };
    let p2 = Progress {
        current_valve: p2_start,
        time_remaining,
    };
    let mut route = Vec::<&'a str>::new();
    let mut cache = HashMap::<(Progress<'a>, Progress<'a>, u32), i32>::new();
    p2_calculate_maximum_releasable_pressure_step(
        valve_map,
        move_map,
        closed_valves,
        &mut cache,
        &p1,
        &p2,
        &mut route,
    )
}

fn get_closed_valves<'a>(valve_map: &HashMap<&'a str, Valve<'a>>) -> Vec<&'a str> {
    let mut valves_to_open = Vec::<&'a str>::new();
    for v in valve_map.values() {
        if v.pressure_released > 0 {
            valves_to_open.push(v.name);
        }
    }
    valves_to_open
}

fn calculate_maximum_releasable_pressure<'a>(
    map: &HashMap<&'a str, Valve<'a>>,
    move_map: &HashMap<(&'a str, &'a str), i32>,
    closed_valves: &Vec<&'a str>,
    time_remaining: i32,
    current_valve: &'a str,
) -> (i32, Option<&'a str>) {
    let mut maximum_value_found = 0;
    let mut valve_picked: Option<&'a str> = None;
    for i in 0..closed_valves.len() {
        let next_valve = closed_valves[i];
        let mut closed_valves = closed_valves.clone();
        closed_valves.swap_remove(i);
        let distance = move_map[&(current_valve, next_valve)];
        if time_remaining < distance {
            continue;
        }
        let time_remaining = time_remaining - distance;
        let calc = calculate_maximum_releasable_pressure(
            map,
            move_map,
            &closed_valves,
            time_remaining,
            next_valve,
        );
        let calc_released = calc.0 + time_remaining * map[next_valve].pressure_released;
        if calc_released > maximum_value_found {
            maximum_value_found = calc_released.max(maximum_value_found);
            valve_picked = Some(next_valve);
        }
    }
    (maximum_value_found, valve_picked)
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    // let (_remaining, valves) = parse_input(TEST_INPUT).unwrap();
    let (_remaining, valves) = parse_input(&input).unwrap();
    let valve_map = calculate_initial_map(valves);
    let move_map = calculate_map(&valve_map);
    let mut closed_valves = get_closed_valves(&valve_map);
    let max_releasable_pressure =
        calculate_maximum_releasable_pressure(&valve_map, &move_map, &mut closed_valves, 30, "AA");
    println!("{}", max_releasable_pressure.0);

    let max_releasable_pressure = p2_calculate_maximum_releasable_pressure(
        &valve_map,
        &move_map,
        &mut closed_valves,
        26,
        "AA",
        "AA",
    );
    println!("{}", max_releasable_pressure);
}
