use std::collections::HashMap;

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_simple_total_sides() {
        let (_, cubes) = parse_input(TEST_INPUT1).unwrap();
        let mut cubes = cube_to_dimensions(cubes);
        assert_eq!(10, count_exposed_sides_part1(&mut cubes));
    }

    #[test]
    fn test_part1_total_sides() {
        let (_, cubes) = parse_input(TEST_INPUT2).unwrap();
        let mut cubes = cube_to_dimensions(cubes);
        assert_eq!(64, count_exposed_sides_part1(&mut cubes));
    }
    #[test]
    fn test_part2_total_sides() {
        let (_, cubes) = parse_input(TEST_INPUT2).unwrap();
        let mut cubes = cube_to_dimensions(cubes);
        assert_eq!(58, count_exposed_sides_part2(&mut cubes));
    }
}

const TEST_INPUT1: &str = "1,1,1
2,1,1";
const TEST_INPUT2: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

#[derive(Debug)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
    exposed_sides: usize,
}

#[derive(Debug, PartialEq)]
enum ExposedToWater {
    Calculating,
    Exposed,
    Unexposed,
    Rock,
}

fn parse_input<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<Cube>> {
    use nom::character::complete::char;
    use nom::character::complete::newline;
    use nom::character::complete::u32;
    use nom::multi::separated_list0;

    let (remaining, cubes) = separated_list0(newline, separated_list0(char(','), u32))(input)?;

    let cubes = cubes
        .iter()
        .map(|v| Cube {
            x: v[0] as usize + 1,
            y: v[1] as usize + 1,
            z: v[2] as usize + 1,
            exposed_sides: 0,
        })
        .collect::<Vec<Cube>>();
    Ok((remaining, cubes))
}

fn has_square_part1(
    cubes: &HashMap<(usize, usize, usize), Cube>,
    x: usize,
    y: usize,
    z: usize,
) -> usize {
    if cubes.contains_key(&(x, y, z)) {
        0
    } else {
        1
    }
}

fn has_square_part2(
    aircubes: &HashMap<(usize, usize, usize), ExposedToWater>,
    x: usize,
    y: usize,
    z: usize,
) -> usize {
    let etw = aircubes.get(&(x, y, z));
    if let Some(etw) = etw {
        if *etw == ExposedToWater::Exposed {
            1
        } else {
            0
        }
    } else {
        0
    }
}

fn count_exposed_sides_part1(cubes: &mut HashMap<(usize, usize, usize), Cube>) -> usize {
    let mut sides_count = 0;
    let mut ac = HashMap::<&(usize, usize, usize), ExposedToWater>::new();
    for c in cubes.keys() {
        sides_count += has_square_part1(cubes, c.0 - 1, c.1, c.2);
        sides_count += has_square_part1(cubes, c.0 + 1, c.1, c.2);
        sides_count += has_square_part1(cubes, c.0, c.1 - 1, c.2);
        sides_count += has_square_part1(cubes, c.0, c.1 + 1, c.2);
        sides_count += has_square_part1(cubes, c.0, c.1, c.2 - 1);
        sides_count += has_square_part1(cubes, c.0, c.1, c.2 + 1);
    }
    sides_count
}

fn count_exposed_sides_part2(cubes: &mut HashMap<(usize, usize, usize), Cube>) -> usize {
    let mut ac = HashMap::<(usize, usize, usize), ExposedToWater>::new();
    for c in cubes.keys() {
        ac.insert(c.clone(), ExposedToWater::Rock);
    }
    check_if_exposed_to_water(&mut ac);

    let mut sides_count = 0;
    for c in cubes.keys() {
        sides_count += has_square_part2(&ac, c.0 - 1, c.1, c.2);
        sides_count += has_square_part2(&ac, c.0 + 1, c.1, c.2);
        sides_count += has_square_part2(&ac, c.0, c.1 - 1, c.2);
        sides_count += has_square_part2(&ac, c.0, c.1 + 1, c.2);
        sides_count += has_square_part2(&ac, c.0, c.1, c.2 - 1);
        sides_count += has_square_part2(&ac, c.0, c.1, c.2 + 1);
    }
    sides_count
}

fn check_if_exposed_to_water(aircubes: &mut HashMap<(usize, usize, usize), ExposedToWater>) {
    let mv = aircubes.keys().fold((0, 0, 0), |ac, k| {
        (ac.0.max(k.0 + 1), ac.1.max(k.1 + 1), ac.2.max(k.2 + 1))
    });
    let (xmin, xmax, ymin, ymax, zmin, zmax) = (
        0 as isize,
        mv.0 as isize,
        0 as isize,
        mv.1 as isize,
        0 as isize,
        mv.2 as isize,
    );

    for x in 0..xmax {
        insert_if_does_not_exist(
            aircubes,
            (x as usize, ymin as usize, zmin as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (x as usize, ymin as usize, zmax as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (x as usize, ymax as usize, zmax as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (x as usize, ymax as usize, zmin as usize),
            ExposedToWater::Exposed,
        );
    }
    for y in 0..ymax {
        insert_if_does_not_exist(
            aircubes,
            (xmin as usize, y as usize, zmin as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (xmin as usize, y as usize, zmax as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (xmax as usize, y as usize, zmax as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (xmax as usize, y as usize, zmin as usize),
            ExposedToWater::Exposed,
        );
    }
    for z in 0..zmax {
        insert_if_does_not_exist(
            aircubes,
            (xmin as usize, ymin as usize, z as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (xmin as usize, ymin as usize, z as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (xmax as usize, ymax as usize, z as usize),
            ExposedToWater::Exposed,
        );
        insert_if_does_not_exist(
            aircubes,
            (xmax as usize, ymax as usize, z as usize),
            ExposedToWater::Exposed,
        );
    }
    let mut to_process = Vec::<(usize, usize, usize)>::new();
    for (k, v) in aircubes.iter() {
        if *v == ExposedToWater::Exposed {
            to_process.push(k.clone());
        }
    }
    while to_process.len() > 0 {
        let mut to_process_next = Vec::<Option<(usize, usize, usize)>>::new();
        for i in to_process.into_iter() {
            to_process_next.push(insert_if_does_not_exist(
                aircubes,
                (xmin.max(i.0 as isize - 1) as usize, i.1, i.2),
                ExposedToWater::Exposed,
            ));
            to_process_next.push(insert_if_does_not_exist(
                aircubes,
                (xmax.min(i.0 as isize + 1) as usize, i.1, i.2),
                ExposedToWater::Exposed,
            ));
            to_process_next.push(insert_if_does_not_exist(
                aircubes,
                (i.0, (ymin.max(i.1 as isize - 1) as usize), i.2),
                ExposedToWater::Exposed,
            ));
            to_process_next.push(insert_if_does_not_exist(
                aircubes,
                (i.0, (ymax.min(i.1 as isize + 1) as usize), i.2),
                ExposedToWater::Exposed,
            ));
            to_process_next.push(insert_if_does_not_exist(
                aircubes,
                (i.0, i.1, (zmin.max(i.2 as isize - 1) as usize)),
                ExposedToWater::Exposed,
            ));
            to_process_next.push(insert_if_does_not_exist(
                aircubes,
                (i.0, i.1, (zmax.min(i.2 as isize + 1) as usize)),
                ExposedToWater::Exposed,
            ));
        }
        to_process = to_process_next
            .into_iter()
            .filter_map(|v| v)
            .collect::<Vec<(usize, usize, usize)>>();
    }
}

fn insert_if_does_not_exist(
    aircubes: &mut HashMap<(usize, usize, usize), ExposedToWater>,
    key: (usize, usize, usize),
    exposed_to_water: ExposedToWater,
) -> Option<(usize, usize, usize)> {
    if !aircubes.contains_key(&key) {
        aircubes.insert(key, exposed_to_water);
        Some(key)
    } else {
        None
    }
}

fn cube_to_dimensions(cubes: Vec<Cube>) -> HashMap<(usize, usize, usize), Cube> {
    let mut retval = HashMap::<(usize, usize, usize), Cube>::new();
    for c in cubes.into_iter() {
        let key = (c.x, c.y, c.z);
        retval.insert(key, c);
    }
    retval
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
    let (_, cubes) = parse_input(&trimmed_input).unwrap();
    let mut cubes = cube_to_dimensions(cubes);
    println!("{}", count_exposed_sides_part1(&mut cubes));

    let (_, cubes) = parse_input(&trimmed_input).unwrap();
    let mut cubes = cube_to_dimensions(cubes);
    println!("{}", count_exposed_sides_part2(&mut cubes));
}
