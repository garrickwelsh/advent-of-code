use std::fmt::Display;

/// The wind directions to apply for tests
const TEST_DIRECTIONS: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

/// The shapes of the rocks that will fall
const ROCKS: [[[Square; 4]; 4]; 5] = [
    [
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
        [Square::FallingRock, Square::Air, Square::Air, Square::Air],
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
        [
            Square::FallingRock,
            Square::FallingRock,
            Square::FallingRock,
            Square::Air,
        ],
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
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_add_works_iteration_one_test() {
        let mut cm2 = CaveMap::new(&ROCKS, TEST_DIRECTIONS);
        cm2.rocks_to_fall(1);
        assert_eq!(1, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(4, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(6, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(7, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(9, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(10, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(13, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(15, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(17, cm2.current_max_height);
        cm2.rocks_to_fall(1);
        assert_eq!(17, cm2.current_max_height);
    }

    #[test]
    fn check_add_works() {
        let mut cm2 = CaveMap::new(&ROCKS, TEST_DIRECTIONS);
        cm2.rocks_to_fall(2022);
        assert_eq!(3068, cm2.current_max_height);
    }

    #[test]
    fn check_for_part2() {
        let mut cm2 = CaveMap2::new(&ROCKS, TEST_DIRECTIONS);
        cm2.rocks_to_fall(1_000_000_000_000);
        assert_eq!(1514285714288, cm2.current_max_height);
    }
}

/// The square of the [Map] that representing, rock, falling rock, walls and air.
///
/// * Air is '.'
/// * Rock is '#'
/// * FallingRock is '@'
/// * Fall is '-'
/// * Wall is '|'
/// * Corner is '+'
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Square {
    /// .
    Air = b'.',
    /// \#
    Rock = b'#',
    /// @
    FallingRock = b'@',
    /// -
    Floor = b'-',
    /// |
    Wall = b'|',
    /// +
    Corner = b'+',
}

#[derive(Clone, Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct RockShape {
    shape: Vec<Position>,
    square: Square,
}

/// A map to place rocks
#[derive(Debug, PartialEq, Clone)]
struct Map<const W: usize, const H: usize> {
    current_max_height: usize,
    map: [[Square; H]; W],
    falling_rocks: Vec<RockShape>,
    current_rock: Option<RockShape>,
    rock_index: usize,
    directions: Vec<char>,
    directions_index: usize,
    num_rocks_fallen: usize,
}

/// Large map to model how falling rocks land
type CaveMap = Map<9, 7000>;
type CaveMap2 = Map<9, 4_000_000_000_000>;

impl<const W: usize, const H: usize> Display for Map<W, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let from_height = if let Some(rock) = self.current_rock.as_ref() {
            rock.shape.iter().map(|rs| rs.y).max().unwrap()
        } else {
            self.current_max_height
        };
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
    fn new<'a>(rocks: &[[[Square; 4]; 4]; 5], directions: &'a str) -> Self {
        let mut retval = Self {
            current_max_height: 0,
            map: [[Square::Air; H]; W],
            directions: Vec::<char>::with_capacity(directions.len()),
            falling_rocks: Vec::<RockShape>::with_capacity(5),
            current_rock: None,
            rock_index: 0,
            directions_index: 0,
            num_rocks_fallen: 0,
        };

        for c in directions.chars() {
            retval.directions.push(c);
        }

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

        for r in rocks {
            retval.falling_rocks.push(RockShape::new_rock_shape(r));
        }

        retval
    }

    fn add_rock_shape(&mut self) -> RockShape {
        let ri = self.rock_index;
        let base_rock_shape = self.falling_rocks.get(ri).unwrap();
        self.rock_index = (self.rock_index + 1) % self.falling_rocks.len();
        let mut absolute_rock_shape: RockShape = base_rock_shape.clone();

        let target_start = self.current_max_height + 4;
        let xoffset = 3;
        let yoffset = target_start;

        for i in 0..absolute_rock_shape.shape.len() {
            absolute_rock_shape.shape[i].x += xoffset;
            absolute_rock_shape.shape[i].y += yoffset;
        }
        self.paint_falling_rock(&absolute_rock_shape);
        absolute_rock_shape
    }

    fn move_next(&mut self) {
        if self.current_rock.is_none() {
            let rock = self.add_rock_shape();
            self.current_rock = Some(rock);
            // println!("{}", self);
        }
        self.move_rock_shape_wind();
        self.move_rock_shape_gravity();
    }

    fn paint_falling_rock(&mut self, rock_shape: &RockShape) {
        self.paint_square_type_for_rock_shape(rock_shape, Square::FallingRock);
    }

    fn unpaint_falling_rock(&mut self, rock_shape: &RockShape) {
        self.paint_square_type_for_rock_shape(rock_shape, Square::Air);
    }

    fn paint_rock(&mut self, rock_shape: &RockShape) {
        self.paint_square_type_for_rock_shape(rock_shape, Square::Rock);
    }

    fn paint_square_type_for_rock_shape(&mut self, rock_shape: &RockShape, square: Square) {
        let s = &rock_shape.shape;
        for i in 0..s.len() {
            self.map[s[i].x][s[i].y] = square;
        }
    }

    fn move_rock_shape_wind(&mut self) {
        let (rock_shape, mut moved_rock_shape) = {
            let rock_shape = self.current_rock.as_ref().unwrap();
            (rock_shape.clone(), rock_shape.clone())
        };
        self.unpaint_falling_rock(&rock_shape);
        // Move to breeze
        let xoffset: isize = match self.directions[self.directions_index] {
            '<' => -1,
            '>' => 1,
            _ => panic!(),
        };
        for p in moved_rock_shape.shape.iter_mut() {
            p.x = ((p.x as isize) + xoffset) as usize;
        }
        self.directions_index = (self.directions_index + 1) % self.directions.len();
        moved_rock_shape = if moved_rock_shape
            .shape
            .iter()
            .filter(|p| self.map[p.x][p.y] != Square::Air)
            .count()
            > 0
        {
            rock_shape // We hit something so we can't move use the old position
        } else {
            moved_rock_shape // We moved left or right
        };

        self.paint_falling_rock(&moved_rock_shape);
        self.current_rock = Some(moved_rock_shape);
    }

    fn move_rock_shape_gravity(&mut self) {
        let (rock_shape, mut moved_rock_shape) = {
            let rock_shape = self.current_rock.as_ref().unwrap();
            (rock_shape.clone(), rock_shape.clone())
        };
        self.unpaint_falling_rock(&rock_shape);
        for s in moved_rock_shape.shape.iter_mut() {
            s.y -= 1;
        }
        if moved_rock_shape
            .shape
            .iter()
            .filter(|p| self.map[p.x][p.y] != Square::Air)
            .count()
            > 0
        {
            // println!("{}", self.current_max_height);
            self.current_max_height = rock_shape
                .shape
                .iter()
                .map(|rs| rs.y)
                .max()
                .unwrap()
                .max(self.current_max_height);
            self.paint_rock(&rock_shape);
            self.current_rock = None;
            self.num_rocks_fallen += 1;
        } else {
            self.paint_falling_rock(&moved_rock_shape);
            self.current_rock = Some(moved_rock_shape);
        }
    }

    fn rocks_to_fall(&mut self, rocks_to_fall: usize) {
        let total_rocks_to_fall_then_stop = self.num_rocks_fallen + rocks_to_fall;
        while self.num_rocks_fallen < total_rocks_to_fall_then_stop {
            self.move_next();
        }
    }
}

impl RockShape {
    fn new_rock_shape(rock: &[[Square; 4]; 4]) -> Self {
        let mut shape = Vec::<Position>::new();
        for x in 0..4 {
            for y in 0..4 {
                if rock[x][y] == Square::FallingRock {
                    shape.push(Position { x, y });
                }
            }
        }
        Self {
            shape,
            square: Square::FallingRock,
        }
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

    // let mut cm = CaveMap::new(&ROCKS, TEST_DIRECTIONS);
    let directions = input.trim();
    let mut cm = CaveMap::new(&ROCKS, directions);
    cm.rocks_to_fall(2022);
    println!("{}", cm.current_max_height);

    let mut cm2 = CaveMap2::new(&ROCKS, directions);
    cm2.rocks_to_fall(1_000_000_000_000);
    println!("{}", cm2.current_max_height);
}
