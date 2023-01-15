#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT1: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn parse_input_test() {
        let area = parse_input(TEST_INPUT1);
        let start = area.start;
        let end = area._end;
        assert_eq!(Coordinate { x: 0, y: 0 }, start);
        assert_eq!(Coordinate { x: 5, y: 2 }, end);
    }

    #[test]
    fn get_minimum_moves() {
        let mut area = parse_input(TEST_INPUT1);
        let num_moves = map_paths_from_start_part1(&mut area);
        assert_eq!(31, num_moves);
    }

    #[test]
    fn get_minimum_moves_from_any_start() {
        let mut area = parse_input(TEST_INPUT1);
        let num_moves = map_paths_from_any_start_part2(&mut area);
        assert_eq!(29, num_moves);
    }
}

#[derive(Debug, Clone)]
struct Square {
    height: u8,
    height_aschar: char,
    min_moves: i32,
    coordinate: Coordinate,
}

#[derive(Debug, Clone)]
struct Area {
    map: Vec<Vec<Square>>,
    start: Coordinate,
    _end: Coordinate,
    xlimit: usize,
    ylimit: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse_input<'a>(input: &'a str) -> Area {
    let mut map = Vec::<Vec<Square>>::new();

    let mut start = Coordinate { x: 0, y: 0 };
    let mut end = Coordinate { x: 0, y: 0 };

    let mut y = 0;
    let mut x = 0;
    for line in input.lines() {
        let mut v = Vec::<Square>::new();
        let bs = line.as_bytes();

        x = 0;
        for b in bs {
            let height = match b {
                0x53u8 => 1u8,
                0x45u8 => 26u8,
                _ => b - 0x60u8,
            };
            if *b == 0x53u8 {
                start = Coordinate { x, y };
            }
            if *b == 0x45u8 {
                end = Coordinate { x, y };
            }
            let s = Square {
                height,
                height_aschar: *b as char,
                min_moves: -1,
                coordinate: Coordinate { x, y },
            };
            v.push(s);
            x += 1;
        }
        map.push(v);
        y += 1;
    }

    Area {
        map,
        start,
        _end: end,
        xlimit: x,
        ylimit: y,
    }
}

fn map_paths_from_any_start_part2(area: &Area) -> usize {
    let map = area.map.clone();
    let starting_points = map
        .into_iter()
        .flatten()
        .filter(|f| f.height == 1)
        .map(|s| s.clone())
        .collect::<Vec<Square>>();
    let mut shortest_path = usize::max_value();
    for starting_point in starting_points {
        let mut working_area = area.clone();
        working_area.start = starting_point.coordinate;
        shortest_path = map_paths_from_start_part1(&mut working_area).min(shortest_path);
    }
    shortest_path
}

fn map_paths_from_start_part1(area: &mut Area) -> usize {
    let mut to_check = Vec::<Coordinate>::new();
    to_check.push(Coordinate {
        x: area.start.x,
        y: area.start.y,
    });

    let mut moves: i32 = 0;
    loop {
        let mut next_check = Vec::<Coordinate>::new();
        while let Some(check) = to_check.pop() {
            let mut square = area.map.get_mut(check.y).unwrap().get_mut(check.x).unwrap();
            // Square has already been looked at don't need to look again.
            if square.min_moves >= 0 {
                continue;
            }
            square.min_moves = moves;
            if square.is_end() {
                return moves as usize;
            }
            // eprintln!("{:?}", square);
            let mut square_next_check = get_next_check(&area.map, &check, area.xlimit, area.ylimit);
            next_check.append(&mut square_next_check);
        }
        to_check = next_check;
        // dbg!(moves, to_check.len());
        // eprintln!("{:?}", to_check);
        moves += 1;
        if to_check.len() == 0 {
            // dbg!(area);
            // dbg!(to_check.clone());
            return usize::max_value();
            // panic!();
        }
    }
}

fn get_next_check(
    map: &Vec<Vec<Square>>,
    to_check: &Coordinate,
    xlimit: usize,
    ylimit: usize,
) -> Vec<Coordinate> {
    let height = map[to_check.y][to_check.x].height;
    // let min_height = height - 1;
    let max_height = height + 1;
    let mut coordinates = [Option::<Coordinate>::None; 4];
    coordinates[0] = get_coordinate(Direction::Left, to_check, xlimit, ylimit);
    coordinates[1] = get_coordinate(Direction::Right, to_check, xlimit, ylimit);
    coordinates[2] = get_coordinate(Direction::Up, to_check, xlimit, ylimit);
    coordinates[3] = get_coordinate(Direction::Down, to_check, xlimit, ylimit);

    coordinates
        .iter()
        .filter(|c| c.is_some())
        .map(|c| c.unwrap())
        .filter(|c| map[c.y][c.x].min_moves < 0 && map[c.y][c.x].height <= max_height)
        .collect::<Vec<Coordinate>>()
}

fn get_coordinate(
    direction: Direction,
    location: &Coordinate,
    xlimit: usize,
    ylimit: usize,
) -> Option<Coordinate> {
    match direction {
        Direction::Left => {
            if location.x == 0 {
                None
            } else {
                Some(Coordinate {
                    x: location.x - 1,
                    y: location.y,
                })
            }
        }
        Direction::Up => {
            if location.y == 0 {
                None
            } else {
                Some(Coordinate {
                    x: location.x,
                    y: location.y - 1,
                })
            }
        }
        Direction::Right => {
            if location.x >= xlimit - 1 {
                None
            } else {
                Some(Coordinate {
                    x: location.x + 1,
                    y: location.y,
                })
            }
        }
        Direction::Down => {
            if location.y >= ylimit - 1 {
                None
            } else {
                Some(Coordinate {
                    x: location.x,
                    y: location.y + 1,
                })
            }
        }
    }
}

impl Square {
    fn _is_start(&self) -> bool {
        self.height_aschar == 'S'
    }
    fn is_end(&self) -> bool {
        self.height_aschar == 'E'
    }
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut area = parse_input(&input);
    let area2 = area.clone();
    let num_moves = map_paths_from_start_part1(&mut area);
    println!("{}", num_moves);

    let num_moves = map_paths_from_any_start_part2(&area2);
    println!("{}", num_moves);
}
