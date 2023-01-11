use std::collections::HashMap;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn find_visited_count_test() {
        let (_, moves) = read_moves(TEST_INPUT).unwrap();
        let visited_count = find_visited_count(&moves);
        assert_eq!(13, visited_count);
    }
}

#[derive(Debug)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
struct Move {
    direction: MoveDirection,
    spaces: i32,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn read_move(input: &str) -> nom::IResult<&str, Move> {
    use nom::bytes::complete::is_a;
    use nom::character::complete::{char, digit0};
    use nom::sequence::separated_pair;

    let (remaining, result) = separated_pair(is_a("LRUD"), char(' '), digit0)(input)?;

    let direction = match result.0 {
        "L" => MoveDirection::Left,
        "R" => MoveDirection::Right,
        "U" => MoveDirection::Up,
        "D" => MoveDirection::Down,
        _ => panic!(),
    };
    let spaces = result.1.parse::<i32>().unwrap();

    Ok((remaining, Move { direction, spaces }))
}

fn read_moves(input: &str) -> nom::IResult<&str, Vec<Move>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;
    let (remaining, result) = separated_list0(newline, read_move)(input)?;
    println!("{}", remaining);

    Ok((remaining, result))
}

fn find_visited_count(moves: &Vec<Move>) -> usize {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };

    let mut hash_map = HashMap::<Position, &str>::new();

    let tstore = tail.clone();
    hash_map.insert(tstore, "value stored");

    for m in moves {
        let mut spaces = m.spaces;
        while spaces > 0 {
            match m.direction {
                MoveDirection::Up => head.y += 1,
                MoveDirection::Down => head.y -= 1,
                MoveDirection::Right => head.x += 1,
                MoveDirection::Left => head.x -= 1,
                // _ => panic!(),
            };
            spaces -= 1;
            if head.x.abs_diff(tail.x) > 1 || head.y.abs_diff(tail.y) > 1 {
                let movex = head.x - tail.x;
                let movex = if movex > 0 {
                    1
                } else if movex < 0 {
                    -1
                } else {
                    0
                };
                let movey = head.y - tail.y;
                let movey = if movey > 0 {
                    1
                } else if movey < 0 {
                    -1
                } else {
                    0
                };
                tail.x += movex;
                tail.y += movey;
                let tstore = tail.clone();
                hash_map.insert(tstore, "value stored");
            }
        }
    }
    // dbg!(hash_map);
    // 0
    hash_map.len()
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let (_, moves) = read_moves(&input).unwrap();

    let tail_visited = find_visited_count(&moves);
    println!("{}", tail_visited);

    // let mut x = 0i32;
    // let mut y = 0i32;
    // let (mut minx, mut miny, mut maxx, mut maxy) = (0i32, 0i32, 0i32, 0i32);

    // println!("{:?}", moves);
    // for m in moves {
    //     match m.direction {
    //         MoveDirection::Up => y += m.spaces as i32,
    //         MoveDirection::Down => y -= m.spaces as i32,
    //         MoveDirection::Right => x += m.spaces as i32,
    //         MoveDirection::Left => x -= m.spaces as i32,
    //         _ => panic!(),
    //     };
    //     maxx = std::cmp::max(maxx, x as i32);
    //     minx = std::cmp::min(minx, x as i32);
    //     maxy = std::cmp::max(maxy, y as i32);
    //     miny = std::cmp::min(miny, y as i32);
    // }
    // println!("min ({}, {}) max({}, {})", minx, miny, maxx, maxy);

    // println!("Hello, world!");
}
