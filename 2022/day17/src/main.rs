use std::fmt::Display;

const TEST_DIRECTIONS: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

const ROCKS: [[[Square; 4]; 4]; 5] = [
    [
        [
            Square::FallingRock,
            Square::FallingRock,
            Square::Air,
            Square::Air,
        ],
        [
            Square::FallingRock,
            Square::FallingRock,
            Square::Air,
            Square::Air,
        ],
        [Square::Air, Square::Air, Square::Air, Square::Air],
        [Square::Air, Square::Air, Square::Air, Square::Air],
    ],
    [
        [
            Square::FallingRock,
            Square::FallingRock,
            Square::FallingRock,
            Square::FallingRock,
        ],
        [Square::Air, Square::Air, Square::Air, Square::Air],
        [Square::Air, Square::Air, Square::Air, Square::Air],
        [Square::Air, Square::Air, Square::Air, Square::Air],
    ],
    [
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
        [
            Square::FallingRock,
            Square::FallingRock,
            Square::FallingRock,
            Square::Air,
        ],
        [Square::Air, Square::Air, Square::Air, Square::Air],
    ],
    [
        [Square::Air, Square::FallingRock, Square::Air, Square::Air],
        [
            Square::FallingRock,
            Square::FallingRock,
            Square::FallingRock,
            Square::Air,
        ],
        [Square::Air, Square::FallingRock, Square::Air, Square::Air],
        [Square::Air, Square::Air, Square::Air, Square::Air],
    ],
    [
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
    ],
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_height_test() {
        todo!();
        // get_height_for_wind_directions(CaveMap, "");
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Square {
    Air = b'.',
    Rock = b'#',
    FallingRock = b'@',
    Floor = b'-',
    Wall = b'|',
    Corner = b'+',
}

struct Map<const W: usize, const H: usize> {
    current_max_height: usize,
    map: [[Square; H]; W],
}

type CaveMap = Map<9, 7000>;

impl<const W: usize, const H: usize> Display for Map<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_height = self.current_max_height + 3;
        let mut output = String::with_capacity(W * from_height + from_height);
        for y in (0..=from_height).rev() {
            for x in 0..self.map.len() {
                let b: u8 = self.map[x][y] as u8;
                output.push(b as char);
            }
            output.push('\n');
        }
        write!(f, "{}", output)
    }
}

impl<const W: usize, const H: usize> Map<W, H> {
    fn new() -> Self {
        let mut retval = Self {
            current_max_height: 0,
            map: [[Square::Air; H]; W],
        };

        for x in 0..W {
            retval.map[x][0] = Square::Floor;
        }
        let x = W - 1;
        for y in 0..H {
            retval.map[0][y] = Square::Wall;
            retval.map[x][y] = Square::Wall;
        }
        retval.map[0][0] = Square::Corner;
        retval.map[x][0] = Square::Corner;

        retval
    }

    fn add_rock(&mut self, rock: &[[Square; 4]; 4]) {
        let target_start = self.current_max_height + 3;
        for x in 3..7 {
            for y in target_start..target_start + 4 {
                self.map[x][y] = rock[x - 3][y - target_start];
            }
        }
        self.current_max_height += 4; // TODO Fix this up.
    }
}

fn get_height_for_wind_directions(cave_map: CaveMap, _directions: &str) -> u32 {
    todo!();
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let mut cm = CaveMap::new();
    cm.add_rock(&ROCKS[0]);
    cm.add_rock(&ROCKS[4]);
    println!("{}", cm);
    get_height_for_wind_directions(cm, "");
    println!("Hello, world!");
}
