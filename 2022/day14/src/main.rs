#[cfg(test)]
mod test {
    use super::*;

    pub(super) const TEST_LINE1: &str = "498,4 -> 498,6 -> 496,6";
    pub(super) const TEST_LINE2: &str = "503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn first_part1_test() {
        let (remaining, result) = parse_line(TEST_LINE1).unwrap();
        assert_eq!("", remaining);
        assert_eq!(
            true,
            result.iter().find(|e| e.x == 498 && e.y == 4).is_some()
        );
    }
}

#[derive(Debug, PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
    SandFalling,
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
    material: Material,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Coordinate>>,
    xoffset: usize,
    yoffset: usize,
    xsize: usize,
    ysize: usize,
}

fn parse_line(input: &str) -> nom::IResult<&str, Vec<Coordinate>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::digit1;
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;

    let (remaining, result) =
        separated_list0(tag(" -> "), separated_pair(digit1, tag(","), digit1))(input)?;
    let coordinates = result
        .iter()
        .map(|c| Coordinate {
            x: c.0.parse::<usize>().unwrap(),
            y: c.1.parse::<usize>().unwrap(),
            material: Material::Rock,
        })
        .collect::<Vec<Coordinate>>();
    Ok((remaining, coordinates))
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<Vec<Coordinate>>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;

    let (remaining, result) = separated_list0(newline, parse_line)(input)?;
    Ok((remaining, result))
}

fn create_matrix(rock_lines: Vec<Vec<Coordinate>>) -> Map {
    let xmin = rock_lines
        .iter()
        .map(|m| m.iter().map(|c| c.x).min().unwrap_or(usize::MAX))
        .min()
        .unwrap();
    let xmax = rock_lines
        .iter()
        .map(|m| m.iter().map(|c| c.x).max().unwrap_or(usize::MIN))
        .max()
        .unwrap();
    let _ymin = rock_lines
        .iter()
        .map(|m| m.iter().map(|c| c.y).min().unwrap_or(usize::MAX))
        .min()
        .unwrap();
    let ymax = rock_lines
        .iter()
        .map(|m| m.iter().map(|c| c.y).max().unwrap_or(usize::MIN))
        .max()
        .unwrap();

    dbg!(xmin, xmax);
    let xsize = (xmax - xmin + 3) as usize;
    // let ysize = (ymax - ymin + 3) as usize;
    let ysize = (ymax + 3) as usize;
    let xoffset = xmin - 1;
    let yoffset = 0; //ymin - 1;

    let mut map = Vec::<Vec<Coordinate>>::new();
    for i in 0..xsize {
        map.push(Vec::<Coordinate>::new());
        for j in 0..ysize {
            map[i].push(Coordinate {
                x: i + xoffset,
                y: j + yoffset,
                material: Material::Air,
            });
        }
    }

    for coords in rock_lines {
        for cwindow in coords.windows(2) {
            let xiter = if cwindow[0].x < cwindow[1].x {
                cwindow[0].x..=cwindow[1].x
            } else {
                cwindow[1].x..=cwindow[0].x
            };
            let yiter = if cwindow[0].y < cwindow[1].y {
                cwindow[0].y..=cwindow[1].y
            } else {
                cwindow[1].y..=cwindow[0].y
            };
            for x in xiter {
                let yiterc = yiter.clone();
                for y in yiterc {
                    // println!("({},{})", x - xoffset, y - yoffset);
                    map[x - xoffset][y - yoffset].material = Material::Rock;
                }
            }
        }
    }

    Map {
        map,
        xoffset,
        yoffset,
        xsize,
        ysize,
    }
}

fn create_matrix_part2(rock_lines: Vec<Vec<Coordinate>>) -> Map {
    let xmin = rock_lines
        .iter()
        .map(|m| m.iter().map(|c| c.x).min().unwrap_or(usize::MAX))
        .min()
        .unwrap();
    let xmax = rock_lines
        .iter()
        .map(|m| m.iter().map(|c| c.x).max().unwrap_or(usize::MIN))
        .max()
        .unwrap();
    let _ymin = rock_lines
        .iter()
        .map(|m| m.iter().map(|c| c.y).min().unwrap_or(usize::MAX))
        .min()
        .unwrap();
    let ymax = rock_lines
        .iter()
        .map(|m| m.iter().map(|c| c.y).max().unwrap_or(usize::MIN))
        .max()
        .unwrap();

    let xsize = (ymax * 3) as usize;
    // let ysize = (ymax - ymin + 3) as usize;
    let ysize = (ymax + 8) as usize;
    let xoffset = xmin - ymax;
    let yoffset = 0; //ymin - 1;

    let mut map = Vec::<Vec<Coordinate>>::new();
    for i in 0..xsize {
        map.push(Vec::<Coordinate>::new());
        for j in 0..ysize {
            map[i].push(Coordinate {
                x: i + xoffset,
                y: j + yoffset,
                material: Material::Air,
            });
        }
    }

    for coords in rock_lines {
        for cwindow in coords.windows(2) {
            let xiter = if cwindow[0].x < cwindow[1].x {
                cwindow[0].x..=cwindow[1].x
            } else {
                cwindow[1].x..=cwindow[0].x
            };
            let yiter = if cwindow[0].y < cwindow[1].y {
                cwindow[0].y..=cwindow[1].y
            } else {
                cwindow[1].y..=cwindow[0].y
            };
            for x in xiter {
                let yiterc = yiter.clone();
                for y in yiterc {
                    // println!("({},{})", x - xoffset, y - yoffset);
                    map[x - xoffset][y - yoffset].material = Material::Rock;
                }
            }
        }
    }

    let y = ymax + 2;
    for x in 0..xsize {
        map[x][y].material = Material::Rock;
    }

    Map {
        map,
        xoffset,
        yoffset,
        xsize,
        ysize,
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                match self.map[x][y].material {
                    Material::Air => output.push('.'),
                    Material::Rock => output.push('#'),
                    Material::Sand => output.push('o'),
                    Material::SandFalling => output.push('+'),
                }
            }
            output.push('\n');
        }
        write!(f, "{}", &output)
    }
}

enum Grain {
    Settled(usize, usize),
    Falling(usize, usize),
    Abyss,
}

impl Map {
    fn simulate_falling_sand(&mut self, startx: usize, starty: usize) -> usize {
        let mut settled_sand_count = 0;
        let tstartx = startx;
        let tstarty = starty;

        self.map[tstartx][tstarty].material = Material::SandFalling;

        let mut grain = self.move_next(tstartx, tstarty);
        let mut x = tstartx;
        let mut y = tstarty;
        loop {
            (x, y) = match grain {
                Grain::Settled(x, y) => {
                    settled_sand_count += 1;
                    if x == tstartx && y == tstarty {
                        break;
                    }
                    self.map[tstartx][tstarty].material = Material::SandFalling;
                    (tstartx, tstarty)
                }
                Grain::Falling(ux, uy) => (ux, uy),
                Grain::Abyss => {
                    // println!("Abyss");
                    break;
                }
            };
            grain = self.move_next(x, y);
            // println!("{}", self);
        }
        settled_sand_count
    }

    fn move_next(&mut self, x: usize, y: usize) -> Grain {
        if y >= self.ysize - 1 {
            println!("Test");
            self.map[x][y].material = Material::Air;
            return Grain::Abyss;
        }

        let mut testx = x;
        let testy = y + 1;
        let mut can_move = false;

        can_move = self.can_move(testx, testy);
        if !can_move {
            testx -= 1;
            can_move = self.can_move(testx, testy);
        }
        if !can_move {
            testx += 2;
            can_move = self.can_move(testx, testy);
        }
        if !can_move {
            self.map[x][y].material = Material::Sand;
            return Grain::Settled(x, y);
        }

        self.map[x][y].material = Material::Air;
        self.map[testx][testy].material = Material::SandFalling;
        Grain::Falling(testx, testy)
    }

    fn can_move(&self, x: usize, y: usize) -> bool {
        // dbg!(x, y);
        self.map[x][y].material == Material::Air
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

    println!("Hello, world!");
    // const LINE_TO_DISPLAY: &str = "498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9";
    // let (_, pinput) = parse_input(LINE_TO_DISPLAY).unwrap();

    const LINE_TO_DISPLAY: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    // let (_, pinput) = parse_input(LINE_TO_DISPLAY).unwrap();
    let (_, pinput) = parse_input(&input).unwrap();
    let mut matrix = create_matrix_part2(pinput);
    // let mut matrix = create_matrix(pinput);
    let output = matrix.simulate_falling_sand(500 - matrix.xoffset, 0);
    // dbg!(matrix);
    println!("{}", matrix);
    // dbg!(matrix.xoffset, matrix.yoffset, matrix.xsize, matrix.ysize);
    println!("{}", output);
}
