mod input;

use input::PUZZLE_INPUT_STR;

#[cfg(test)]
mod test {
    use super::*;
    const TEST_STR: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn parse_test() {
        use Movement;

        let (_, movements) = parse_lines(TEST_STR).unwrap();
        assert_eq!(movements[0], Movement::Forward(10));
        assert_eq!(movements[1], Movement::Move(Direction::North, 3));
        assert_eq!(movements[2], Movement::Forward(7));
        assert_eq!(movements[3], Movement::Turn(TurnDirection::Right, 1));
        assert_eq!(movements[4], Movement::Forward(11));
    }

    #[test]
    fn calculate_manhattan_distance_test() {
        let (_, movements) = parse_lines(TEST_STR).unwrap();

        let mut ship = Ship {
            direction: Direction::East,
            position_x: 0,
            position_y: 0,
        };
        for m in movements {
            ship = ship.action_movement(m);
        }
        let md = ship.get_manhattan_distance(0, 0);
        assert_eq!(25, md);
    }

    #[test]
    fn calculate_ship_movement_with_manhattan_distance_test() {
        let (_, movements) = parse_lines(TEST_STR).unwrap();

        let mut ship = ShipWithWayPoint {
            x: 0,
            y: 0,
            wposition_x: 10,
            wposition_y: 1,
        };
        for m in movements {
            ship = ship.action_movement(m);
        }
        let md = ship.get_manhattan_distance(0, 0);
        assert_eq!(286, md);
    }

    #[test]
    fn calculate_each_movement_part2() {
        let (_, movements) = parse_lines(TEST_STR).unwrap();
        let mut ship = ShipWithWayPoint {
            x: 0,
            y: 0,
            wposition_x: 10,
            wposition_y: 1,
        };
        ship = ship.action_movement(movements[0]);
        assert_eq!(100, ship.x);
        assert_eq!(10, ship.y);
        ship = ship.action_movement(movements[1]);
        assert_eq!(100, ship.x);
        assert_eq!(10, ship.y);
        ship = ship.action_movement(movements[2]);
        assert_eq!(170, ship.x);
        assert_eq!(38, ship.y);
        ship = ship.action_movement(movements[3]);
        assert_eq!(170, ship.x);
        assert_eq!(38, ship.y);
        ship = ship.action_movement(movements[4]);
        assert_eq!(4, ship.wposition_x);
        assert_eq!(-10, ship.wposition_y);
        assert_eq!(214, ship.x);
        assert_eq!(-72, ship.y);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Movement {
    Move(Direction, i32),
    Turn(TurnDirection, i32),
    Forward(i32),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Ship {
    direction: Direction,
    position_x: i32,
    position_y: i32,
}

impl Ship {
    fn move_in_direction(&self, direction: Direction, distance: i32) -> Self {
        match direction {
            Direction::North => Self {
                direction: self.direction,
                position_x: self.position_x,
                position_y: self.position_y + distance,
            },
            Direction::South => Self {
                direction: self.direction,
                position_x: self.position_x,
                position_y: self.position_y - distance,
            },
            Direction::East => Self {
                direction: self.direction,
                position_x: self.position_x + distance,
                position_y: self.position_y,
            },
            Direction::West => Self {
                direction: self.direction,
                position_x: self.position_x - distance,
                position_y: self.position_y,
            },
        }
    }

    fn action_movement(&self, movement: Movement) -> Self {
        match movement {
            Movement::Move(direction, distance) => self.move_in_direction(direction, distance),
            Movement::Forward(distance) => {
                let direction = self.direction;
                self.move_in_direction(direction, distance)
            }
            Movement::Turn(turn_direction, turns) => self.turn(turn_direction, turns),
        }
    }

    fn turn(&self, turn_direction: TurnDirection, turns: i32) -> Self {
        let mut current_direction = self.direction;
        for _ in 0..turns {
            current_direction = current_direction.turn(turn_direction);
        }
        Self {
            direction: current_direction,
            position_x: self.position_x,
            position_y: self.position_y,
        }
    }

    fn get_manhattan_distance(&self, position_x: i32, position_y: i32) -> i32 {
        (position_x - self.position_x).abs() + (position_y - self.position_y).abs()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct ShipWithWayPoint {
    x: i32,
    y: i32,
    wposition_x: i32,
    wposition_y: i32,
}

impl ShipWithWayPoint {
    fn move_in_direction(&self, direction: Direction, distance: i32) -> Self {
        match direction {
            Direction::North => Self {
                x: self.x,
                y: self.y,
                wposition_x: self.wposition_x,
                wposition_y: self.wposition_y + distance,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y,
                wposition_x: self.wposition_x,
                wposition_y: self.wposition_y - distance,
            },
            Direction::East => Self {
                x: self.x,
                y: self.y,
                wposition_x: self.wposition_x + distance,
                wposition_y: self.wposition_y,
            },
            Direction::West => Self {
                x: self.x,
                y: self.y,
                wposition_x: self.wposition_x - distance,
                wposition_y: self.wposition_y,
            },
        }
    }

    fn action_movement(&self, movement: Movement) -> Self {
        match movement {
            Movement::Move(direction, distance) => self.move_in_direction(direction, distance),
            Movement::Forward(distance) => {
                let mut dest = self.clone();
                for _ in 0..distance {
                    dest = Self {
                        x: dest.wposition_x + dest.x,
                        wposition_x: dest.wposition_x,
                        y: dest.wposition_y + dest.y,
                        wposition_y: dest.wposition_y,
                    };
                }
                dest
            }
            Movement::Turn(turn_direction, turns) => self.turn(turn_direction, turns),
        }
    }

    fn turn(&self, turn_direction: TurnDirection, turns: i32) -> Self {
        let mut position_x = self.wposition_x;
        let mut position_y = self.wposition_y;

        for _ in 0..turns {
            (position_x, position_y) = match turn_direction {
                TurnDirection::Left => (-position_y, position_x),
                TurnDirection::Right => (position_y, -position_x),
            };
        }
        Self {
            x: self.x,
            y: self.y,
            wposition_x: position_x,
            wposition_y: position_y,
        }
    }

    fn get_manhattan_distance(&self, position_x: i32, position_y: i32) -> i32 {
        (position_x - self.x).abs() + (position_y - self.y).abs()
    }
}

impl Direction {
    fn turn(&self, turn_direction: TurnDirection) -> Self {
        if turn_direction == TurnDirection::Left {
            match *self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            }
        } else {
            match *self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            }
        }
    }
}

fn parse_lines<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<Movement>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;

    separated_list0(newline, parse_line)(input)
}

fn parse_line<'a>(input: &'a str) -> nom::IResult<&'a str, Movement> {
    use nom::character::complete::anychar;
    use nom::character::complete::digit1;

    let (input, command) = anychar(input)?;
    let (input, digits) = digit1(input)?;

    let number = digits.parse::<i32>().unwrap();

    let movement = match command {
        'L' => Movement::Turn(TurnDirection::Left, number / 90),
        'R' => Movement::Turn(TurnDirection::Right, number / 90),
        'F' => Movement::Forward(number),
        'N' => Movement::Move(Direction::North, number),
        'S' => Movement::Move(Direction::South, number),
        'E' => Movement::Move(Direction::East, number),
        'W' => Movement::Move(Direction::West, number),
        _ => panic!(),
    };
    Ok((input, movement))
}

fn main() {
    let (_, movements) = parse_lines(PUZZLE_INPUT_STR).unwrap();

    let mut ship = Ship {
        direction: Direction::East,
        position_x: 0,
        position_y: 0,
    };
    for m in movements.clone() {
        ship = ship.action_movement(m);
    }
    let md = ship.get_manhattan_distance(0, 0);
    println!("{}", md);

    let mut ship = ShipWithWayPoint {
        x: 0,
        y: 0,
        wposition_x: 10,
        wposition_y: 1,
    };
    for m in movements {
        ship = ship.action_movement(m);
    }
    let md = ship.get_manhattan_distance(0, 0);
    println!("{}", md);
}
