#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn todo() {
        const READING: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
        let (_, reading) = parse_reading(READING).unwrap();
        assert_eq!(2, reading.sensor.x);
        assert_eq!(18, reading.sensor.y);
        assert_eq!(-2, reading.beacon.x);
        assert_eq!(15, reading.beacon.y);
    }
}

const SENSOR_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

#[derive(Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct Reading {
    sensor: Position,
    beacon: Position,
    manhattan_distance: i32,
}

impl Position {
    fn manhattan_distance(&self, position: &Position) -> i32 {
        (position.x - self.x).abs() + (position.y - self.y).abs()
    }
}

impl Reading {
    fn within_reading(&self, position: &Position) -> bool {
        self.sensor.manhattan_distance(position) <= self.manhattan_distance
            && !(self.beacon.x == position.x && self.beacon.y == position.y)
    }
    fn within_reading_2(&self, position: &Position) -> bool {
        self.sensor.manhattan_distance(position) <= self.manhattan_distance
    }
}

fn parse_position(input: &str) -> nom::IResult<&str, Position> {
    use nom::bytes::complete::tag;
    use nom::character::complete::i32;
    use nom::sequence::pair;
    use nom::sequence::preceded;

    let (remaining, coord) = pair(preceded(tag("x="), i32), preceded(tag(", y="), i32))(input)?;
    Ok((
        remaining,
        Position {
            x: coord.0,
            y: coord.1,
        },
    ))
}

fn parse_reading(input: &str) -> nom::IResult<&str, Reading> {
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    use nom::bytes::complete::tag;
    use nom::sequence::pair;
    use nom::sequence::preceded;

    let (remaining, result) = pair(
        preceded(tag("Sensor at "), parse_position),
        preceded(tag(": closest beacon is at "), parse_position),
    )(input)?;
    let manhattan_distance = result.0.manhattan_distance(&result.1);
    Ok((
        remaining,
        Reading {
            sensor: result.0,
            beacon: result.1,
            manhattan_distance,
        },
    ))
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Reading>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;

    let (remaining, readings) = separated_list0(newline, parse_reading)(input)?;
    Ok((remaining, readings))
}

fn ruled_out(readings: &Vec<Reading>, y: i32) -> i32 {
    let xmin = readings.iter().map(|r| r.sensor.x).min().unwrap();
    let xmax = readings.iter().map(|r| r.sensor.x).max().unwrap();
    let xmin = xmin - readings.iter().map(|r| r.manhattan_distance).max().unwrap() - 1;
    let xmax = xmax + readings.iter().map(|r| r.manhattan_distance).max().unwrap() + 1;
    // let ymax = readings.iter().map(|r| r.sensor.y).max.unwrap();
    let mut position = Position { x: 0, y };
    let mut count = 0;
    for x in xmin..=xmax {
        for r in readings {
            position.x = x;
            if r.within_reading(&position) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn possible_positions(
    readings: &Vec<Reading>,
    xstart: i32,
    ystart: i32,
    xend: i32,
    yend: i32,
) -> Vec<Position> {
    let mut retval = Vec::<Position>::new();
    for x in xstart..=xend {
        for y in ystart..=yend {
            let position = Position { x, y };
            if readings
                .iter()
                .filter(|r| r.within_reading_2(&position))
                .count()
                == 0
            {
                retval.push(position);
            }
        }
    }
    retval
}

#[derive(Debug, Clone, Copy)]
struct LineCover {
    xstart: i32,
    xend: i32,
}

fn calculate_possible_empty_positions(
    readings: &Vec<Reading>,
    y: i32,
    upper_limit: i32,
) -> Option<Position> {
    let mut line_cover = Vec::<LineCover>::new();
    for reading in readings {
        let ydiff = y.abs_diff(reading.sensor.y) as i32;
        let md = if reading.manhattan_distance > ydiff {
            reading.manhattan_distance - ydiff
        } else {
            0
        };
        if md == 0 {
            continue;
        }
        let xstart = reading.sensor.x - md;
        let xend = reading.sensor.x + md;
        line_cover.push(LineCover { xstart, xend });
    }

    for l in line_cover.iter() {
        let x = l.xstart - 1;
        if x >= 0 && x <= upper_limit {
            let p1 = Position { x, y };
            if readings.iter().filter(|r| r.within_reading_2(&p1)).count() == 0 {
                return Some(p1);
            }
        }
        let x = l.xend + 1;
        if x >= 0 && x <= upper_limit {
            let p2 = Position { x, y };
            if readings.iter().filter(|r| r.within_reading_2(&p2)).count() == 0 {
                return Some(p2);
            }
        }
    }
    None
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let (_, readings) = parse_input(SENSOR_INPUT).unwrap();
    println!("{}", ruled_out(&readings, 10));

    let positions = possible_positions(&readings, 0, 0, 20, 20);
    dbg!(positions);
    for i in 0..=20 {
        let positions = calculate_possible_empty_positions(&readings, i, 20);
        if let Some(position) = positions {
            dbg!(position);
        }
    }
    let mut position: Option<Position> = None;
    for i in 0..=20 {
        position = calculate_possible_empty_positions(&readings, i, 20);
        if position.is_some() {
            break;
        }
    }
    let position = position.unwrap();
    println!("{}", position.x * 4000000 + position.y); // Answer: 12555527364986

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let (_, readings) = parse_input(&input).unwrap();

    let mut position: Option<Position> = None;
    for i in 0..=4000000 {
        position = calculate_possible_empty_positions(&readings, i, 4000000);
        if position.is_some() {
            break;
        }
    }
    let position = position.unwrap();
    println!("{}", position.x * 4000000 + position.y); // Answer: 12555527364986
}
