#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn import_map() {
        let (_, (tiles, movement)) = parse_input(TEST_INPUT).unwrap();

        let mut map = Map::new(tiles);
        map.process_movements(&movement);
        println!("{}", map);
        println!("{:?}", map.position);
        assert_eq!(6032, map.calculate_password());
        todo!();
    }
}
const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum TileType {
    Empty,
    Exists,
    Wall,
}

#[derive(Debug)]
struct Tile {
    next_right: Option<(usize, usize)>,
    next_down: Option<(usize, usize)>,
    next_left: Option<(usize, usize)>,
    next_up: Option<(usize, usize)>,
    position: (usize, usize),
    last_movement: Option<Direction>,
    tile_type: TileType,
}

#[derive(Debug)]
struct Movement {
    current_position: (usize, usize),
    current_direction: Direction,
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug)]
enum MovementInstruction {
    Move(usize),
    Turn(Turn),
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Tile>>,
    position: Movement,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            next_right: None,
            next_down: None,
            next_left: None,
            next_up: None,
            position: (0, 0),
            last_movement: None,
            tile_type: TileType::Empty,
        }
    }
}

impl Map {
    fn new(parsed_tiles: Vec<Vec<TileType>>) -> Self {
        let mut map = Self {
            map: Vec::<Vec<Tile>>::new(),
            position: Movement {
                current_position: (0, 0),
                current_direction: Direction::Right,
            },
        };

        let xmax = parsed_tiles.iter().map(|m| m.len()).max().unwrap();
        let ymax = parsed_tiles.len();

        dbg!(xmax, ymax);

        for x in 0..xmax + 2 {
            let v = Vec::<Tile>::new();
            map.map.push(v);
            for _ in 0..ymax + 2 {
                map.map[x].push(Tile::default());
            }
        }

        for y in 0..parsed_tiles.len() {
            for x in 0..parsed_tiles[y].len() {
                map.map[x + 1][y + 1].tile_type = parsed_tiles[y][x];
            }
        }

        for x in 0..xmax + 2 {
            for y in 0..ymax + 2 {
                map.map[x][y].position = (x, y);
            }
        }

        let mut to_wrap = Vec::<(Direction, usize, usize)>::new();

        for x in 1..=xmax {
            for y in 1..=ymax {
                if map.map[x][y].tile_type == TileType::Empty {
                    continue;
                }
                Map::populate_easy_movement(&mut map.map, Direction::Right, x, y, &mut to_wrap);
                Map::populate_easy_movement(&mut map.map, Direction::Left, x, y, &mut to_wrap);
                Map::populate_easy_movement(&mut map.map, Direction::Up, x, y, &mut to_wrap);
                Map::populate_easy_movement(&mut map.map, Direction::Down, x, y, &mut to_wrap);
            }
        }
        for (direction, x, y) in to_wrap.into_iter() {
            match direction {
                Direction::Left => {
                    for tx in (0..map.map.len()).rev() {
                        match map.map[tx][y].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                map.map[x][y].next_left = Some((tx, y));
                                break;
                            }
                        }
                    }
                }
                Direction::Right => {
                    for tx in 0..map.map.len() {
                        match map.map[tx][y].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                map.map[x][y].next_right = Some((tx, y));
                                break;
                            }
                        }
                    }
                }
                Direction::Up => {
                    for ty in (0..map.map[x].len()).rev() {
                        match map.map[x][ty].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                map.map[x][y].next_up = Some((x, ty));
                                break;
                            }
                        }
                    }
                }
                Direction::Down => {
                    for ty in 0..map.map[x].len() {
                        match map.map[x][ty].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                map.map[x][y].next_down = Some((x, ty));
                                break;
                            }
                        }
                    }
                }
            }
        }
        for x in 0..map.map.len() {
            if map.map[x][1].tile_type == TileType::Exists {
                map.position.current_position = (x, 1);
                break;
            }
        }
        map
    }

    fn calculate_password(&self) -> usize {
        self.position.current_direction as usize
            + 1000 * self.position.current_position.1
            + 4 * self.position.current_position.0
    }

    fn process_movements(&mut self, movements: &Vec<MovementInstruction>) {
        for movement in movements.iter() {
            match movement {
                MovementInstruction::Move(steps) => {
                    for _ in 0..*steps {
                        let new_position = match self.position.current_direction {
                            Direction::Right => {
                                self.map[self.position.current_position.0]
                                    [self.position.current_position.1]
                                    .next_right
                            }
                            Direction::Left => {
                                self.map[self.position.current_position.0]
                                    [self.position.current_position.1]
                                    .next_left
                            }
                            Direction::Up => {
                                self.map[self.position.current_position.0]
                                    [self.position.current_position.1]
                                    .next_up
                            }
                            Direction::Down => {
                                self.map[self.position.current_position.0]
                                    [self.position.current_position.1]
                                    .next_down
                            }
                        };
                        if let Some(position) = new_position {
                            self.map[self.position.current_position.0]
                                [self.position.current_position.1]
                                .last_movement = Some(self.position.current_direction);
                            self.position.current_position = position;
                        }
                    }
                }
                MovementInstruction::Turn(turn) => {
                    self.set_new_direction(*turn);
                }
            }
        }
    }

    fn set_new_direction(&mut self, turn: Turn) {
        self.position.current_direction = match self.position.current_direction {
            Direction::Right => match turn {
                Turn::Clockwise => Direction::Down,
                Turn::CounterClockwise => Direction::Up,
            },
            Direction::Left => match turn {
                Turn::Clockwise => Direction::Up,
                Turn::CounterClockwise => Direction::Down,
            },
            Direction::Up => match turn {
                Turn::Clockwise => Direction::Right,
                Turn::CounterClockwise => Direction::Left,
            },
            Direction::Down => match turn {
                Turn::Clockwise => Direction::Left,
                Turn::CounterClockwise => Direction::Right,
            },
        }
    }

    fn populate_easy_movement(
        map: &mut Vec<Vec<Tile>>,
        direction: Direction,
        x: usize,
        y: usize,
        to_wrap: &mut Vec<(Direction, usize, usize)>,
    ) {
        let mut tx = x;
        let mut ty = y;
        match direction {
            Direction::Right => tx += 1,
            Direction::Down => ty += 1,
            Direction::Left => tx -= 1,
            Direction::Up => ty -= 1,
        }
        let tt = map[tx][ty].tile_type;
        if tt == TileType::Exists {
            match direction {
                Direction::Right => {
                    map[x][y].next_right = Some((tx, ty));
                }
                Direction::Left => {
                    map[x][y].next_left = Some((tx, ty));
                }
                Direction::Up => {
                    map[x][y].next_up = Some((tx, ty));
                }
                Direction::Down => {
                    map[x][y].next_down = Some((tx, ty));
                } // _ => panic!(),
            }
        } else if tt == TileType::Empty {
            to_wrap.push((direction, x, y));
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.map[0].len() {
            for x in 0..self.map.len() {
                let t = &self.map[x][y];
                match t.last_movement {
                    None => match t.tile_type {
                        TileType::Empty => s.push(' '),
                        TileType::Exists => s.push('.'),
                        TileType::Wall => s.push('#'),
                    },
                    Some(direction) => match direction {
                        Direction::Left => s.push('<'),
                        Direction::Right => s.push('>'),
                        Direction::Up => s.push('^'),
                        Direction::Down => s.push('v'),
                    },
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn parse_map_line<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<TileType>> {
    use nom::character::complete::one_of;
    use nom::multi::many1;

    let (remaining, mapline) = many1(one_of(" .#"))(input)?;
    let retval = mapline
        .iter()
        .map(|m| match *m {
            ' ' => TileType::Empty,
            '#' => TileType::Wall,
            '.' => TileType::Exists,
            _ => panic!(),
        })
        .collect::<Vec<TileType>>();
    Ok((remaining, retval))
}

fn parse_map<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<Vec<TileType>>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;

    let (remaining, map) = separated_list0(newline, parse_map_line)(input)?;
    Ok((remaining, map))
}
fn parse_movement_turn<'a>(input: &'a str) -> nom::IResult<&'a str, MovementInstruction> {
    use nom::branch::alt;
    use nom::character::complete::char;

    let (remaining, m) = alt((char('R'), char('L')))(input)?;
    let movement_instruction = match m {
        'L' => MovementInstruction::Turn(Turn::CounterClockwise),
        'R' => MovementInstruction::Turn(Turn::Clockwise),
        _ => panic!(),
    };
    Ok((remaining, movement_instruction))
}
fn parse_movement_steps<'a>(input: &'a str) -> nom::IResult<&'a str, MovementInstruction> {
    use nom::character::complete::u32;
    use nom::combinator::map;

    map(u32, |n| MovementInstruction::Move(n as usize))(input)
}
fn parse_movements<'a>(input: &'a str) -> nom::IResult<&'a str, Vec<MovementInstruction>> {
    use nom::branch::alt;
    use nom::multi::many1;

    many1(alt((parse_movement_turn, parse_movement_steps)))(input)
}
fn parse_input<'a>(
    input: &'a str,
) -> nom::IResult<&'a str, (Vec<Vec<TileType>>, Vec<MovementInstruction>)> {
    use nom::character::complete::newline;
    use nom::multi::count;
    use nom::sequence::tuple;

    let (remaining, t) = tuple((parse_map, count(newline, 2), parse_movements))(input)?;
    Ok((remaining, (t.0, t.2)))
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    // let trimmed_input = input.trim();
    let (_, (tile_types, movement_instructions)) = parse_input(&input).unwrap();
    let mut map = Map::new(tile_types);
    map.process_movements(&movement_instructions);
    println!("{}", map);
    println!("{:?}", map.position);
    println!("{}", map.calculate_password());
}
