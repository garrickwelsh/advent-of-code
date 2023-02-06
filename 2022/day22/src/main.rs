#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn import_map_part1() {
        let (_, (tiles, movement)) = parse_input(TEST_INPUT).unwrap();

        let mut map = Map::new_part1(tiles, 4);
        map.process_movements(&movement);
        println!("{}", map);
        println!("{:?}", map.position);
        assert_eq!(6032, map.calculate_password());
    }
    #[test]
    fn import_map_part2() {
        let (_, (tiles, movement)) = parse_input(TEST_INPUT).unwrap();

        let mut map = Map::new_part2(tiles, 4);
        map.process_movements(&movement);
        println!("{}", map);
        println!("{:?}", map.position);
        assert_eq!(5031, map.calculate_password());
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum CubeFace {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

#[derive(Debug)]
struct Tile {
    next_right: Option<(Direction, (usize, usize))>,
    next_down: Option<(Direction, (usize, usize))>,
    next_left: Option<(Direction, (usize, usize))>,
    next_up: Option<(Direction, (usize, usize))>,
    position: (usize, usize),
    last_movement: Option<Direction>,
    tile_type: TileType,
    cube_face: Option<CubeFace>,
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
    face_size: usize,
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
            cube_face: None,
        }
    }
}

impl Map {
    fn new_part1(parsed_tiles: Vec<Vec<TileType>>, face_size: usize) -> Self {
        Map::new(parsed_tiles, face_size, |map, tw| {
            map.calculate_movement_part1(tw)
        })
    }

    fn new_part2(parsed_tiles: Vec<Vec<TileType>>, face_size: usize) -> Self {
        Map::new(parsed_tiles, face_size, |map, tw| {
            map.calculate_movement_part2(tw)
        })
    }
    fn new<F>(parsed_tiles: Vec<Vec<TileType>>, face_size: usize, f: F) -> Self
    where
        F: Fn(&mut Self, Vec<(Direction, usize, usize)>),
    {
        let mut map = Self {
            map: Vec::<Vec<Tile>>::new(),
            position: Movement {
                current_position: (0, 0),
                current_direction: Direction::Right,
            },
            face_size,
        };

        let xmax = parsed_tiles.iter().map(|m| m.len()).max().unwrap();
        let ymax = parsed_tiles.len();

        // dbg!(xmax, ymax);

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
        let face_size = map.face_size;
        let mut faces_found = Vec::<(usize, usize, CubeFace)>::new();
        for y in 0..map.map[0].len() {
            for x in 0..map.map.len() {
                if map.map[x][y].tile_type != TileType::Empty {
                    let ff = faces_found
                        .iter()
                        .filter(|(xf, yf, _)| {
                            x >= *xf && x < (*xf + face_size) && y >= *yf && y < (*yf + face_size)
                        })
                        .map(|(_, _, f)| f.clone())
                        .collect::<Vec<CubeFace>>();
                    debug_assert!(ff.clone().len() <= 1);
                    if ff.clone().len() == 1 {
                        let face = ff.iter().last().unwrap();
                        map.map[x][y].cube_face = Some(face.clone());
                        // dbg!((x, y, face.clone()));
                        continue;
                    }
                    let mut next_face = CubeFace::One;
                    if let Some(face) = faces_found.iter().map(|(_, _, f)| f).max() {
                        println!("max_face");
                        next_face = face.next_face();
                    }
                    map.map[x][y].cube_face = Some(next_face.clone());
                    // dbg!((x, y, next_face.clone()));
                    faces_found.push((x, y, next_face));
                }
                // dbg!(faces_found.clone());
            }
        }
        f(&mut map, to_wrap);
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
                        let m = match self.position.current_direction {
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
                        if let Some((direction, position)) = m {
                            self.map[self.position.current_position.0]
                                [self.position.current_position.1]
                                .last_movement = Some(self.position.current_direction);
                            self.position.current_position = position;
                            self.position.current_direction = direction;
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
                    map[x][y].next_right = Some((direction, (tx, ty)));
                }
                Direction::Left => {
                    map[x][y].next_left = Some((direction, (tx, ty)));
                }
                Direction::Up => {
                    map[x][y].next_up = Some((direction, (tx, ty)));
                }
                Direction::Down => {
                    map[x][y].next_down = Some((direction, (tx, ty)));
                } // _ => panic!(),
            }
        } else if tt == TileType::Empty {
            to_wrap.push((direction, x, y));
        }
    }

    fn calculate_movement_part1(&mut self, to_wrap: Vec<(Direction, usize, usize)>) {
        for (direction, x, y) in to_wrap.into_iter() {
            match direction {
                Direction::Left => {
                    for tx in (0..self.map.len()).rev() {
                        match self.map[tx][y].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                self.map[x][y].next_left = Some((direction, (tx, y)));
                                break;
                            }
                        }
                    }
                }
                Direction::Right => {
                    for tx in 0..self.map.len() {
                        match self.map[tx][y].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                self.map[x][y].next_right = Some((direction, (tx, y)));
                                break;
                            }
                        }
                    }
                }
                Direction::Up => {
                    for ty in (0..self.map[x].len()).rev() {
                        match self.map[x][ty].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                self.map[x][y].next_up = Some((direction, (x, ty)));
                                break;
                            }
                        }
                    }
                }
                Direction::Down => {
                    for ty in 0..self.map[x].len() {
                        match self.map[x][ty].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                self.map[x][y].next_down = Some((direction, (x, ty)));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn calculate_movement_part2(&mut self, to_wrap: Vec<(Direction, usize, usize)>) {
        return;
        todo!();
        for (direction, x, y) in to_wrap.into_iter() {
            match direction {
                Direction::Left => {
                    for tx in (0..self.map.len()).rev() {
                        match self.map[tx][y].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                self.map[x][y].next_left = Some((direction, (tx, y)));
                                break;
                            }
                        }
                    }
                }
                Direction::Right => {
                    for tx in 0..self.map.len() {
                        match self.map[tx][y].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                self.map[x][y].next_right = Some((direction, (tx, y)));
                                break;
                            }
                        }
                    }
                }
                Direction::Up => {
                    for ty in (0..self.map[x].len()).rev() {
                        match self.map[x][ty].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                self.map[x][y].next_up = Some((direction, (x, ty)));
                                break;
                            }
                        }
                    }
                }
                Direction::Down => {
                    for ty in 0..self.map[x].len() {
                        match self.map[x][ty].tile_type {
                            TileType::Empty => {}
                            TileType::Wall => {
                                break;
                            }
                            TileType::Exists => {
                                self.map[x][y].next_down = Some((direction, (x, ty)));
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.map[0].len() {
            for x in 0..self.map.len() {
                let t = &self.map[x][y];
                match t.cube_face {
                    Some(CubeFace::One) => s.push('1'),
                    Some(CubeFace::Two) => s.push('2'),
                    Some(CubeFace::Three) => s.push('3'),
                    Some(CubeFace::Four) => s.push('4'),
                    Some(CubeFace::Five) => s.push('5'),
                    Some(CubeFace::Six) => s.push('6'),
                    _ => s.push(' '),
                }
                // match t.last_movement {
                //     None => match t.tile_type {
                //         TileType::Empty => s.push(' '),
                //         TileType::Exists => s.push('.'),
                //         TileType::Wall => s.push('#'),
                //     },
                //     Some(direction) => match direction {
                //         Direction::Left => s.push('<'),
                //         Direction::Right => s.push('>'),
                //         Direction::Up => s.push('^'),
                //         Direction::Down => s.push('v'),
                //     },
                // }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl CubeFace {
    fn next_face(&self) -> Self {
        match *self {
            CubeFace::One => CubeFace::Two,
            CubeFace::Two => CubeFace::Three,
            CubeFace::Three => CubeFace::Four,
            CubeFace::Four => CubeFace::Five,
            CubeFace::Five => CubeFace::Six,
            CubeFace::Six => panic!(),
        }
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
    let mut map = Map::new_part1(tile_types, 50);
    map.process_movements(&movement_instructions);
    println!("{}", map);
    println!("{:?}", map.position);
    println!("{}", map.calculate_password());
}
