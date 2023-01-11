use std::collections::HashMap;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT_PART1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn find_visited_count_test() {
        let (_, moves) = read_moves(TEST_INPUT_PART1).unwrap();
        let visited_count = find_tail_visited_count::<2>(&moves);
        assert_eq!(13, visited_count);
    }

    const TEST_INPUT_PART2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    #[test]
    fn find_visited_count_test2() {
        let (_, moves) = read_moves(TEST_INPUT_PART2).unwrap();
        let visited_count = find_tail_visited_count::<10>(&moves);
        assert_eq!(36, visited_count);
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

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
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

    Ok((remaining, result))
}

fn move_next_knot(rope: &mut [Position], ahead: usize, behind: usize) -> bool {
    let mut moved = false;
    if rope[ahead].x.abs_diff(rope[behind].x) > 1 || rope[ahead].y.abs_diff(rope[behind].y) > 1 {
        let movex = rope[ahead].x - rope[behind].x;
        let movex = if movex > 0 {
            1
        } else if movex < 0 {
            -1
        } else {
            0
        };
        let movey = rope[ahead].y - rope[behind].y;
        let movey = if movey > 0 {
            1
        } else if movey < 0 {
            -1
        } else {
            0
        };
        rope[behind].x += movex;
        rope[behind].y += movey;
        moved = (movex | movey) != 0;
    }
    moved
}
fn find_tail_visited_count<const N: usize>(moves: &Vec<Move>) -> usize {
    let mut rope = [Position { x: 0, y: 0 }; N];

    let mut hash_map = HashMap::<Position, &str>::new();

    // store tail starting position
    hash_map.insert(rope[N - 1], "value stored");

    for m in moves {
        let mut spaces = m.spaces;
        while spaces > 0 {
            match m.direction {
                MoveDirection::Up => rope[0].y += 1,
                MoveDirection::Down => rope[0].y -= 1,
                MoveDirection::Right => rope[0].x += 1,
                MoveDirection::Left => rope[0].x -= 1,
                // _ => panic!(),
            };
            spaces -= 1;
            for i in 0..N - 1 {
                if !move_next_knot(&mut rope, i, i + 1) {
                    break;
                } else if i == N - 2 {
                    // Tail has moved then store position
                    hash_map.insert(rope[i + 1], "Some value");
                }
            }
        }
    }
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

    let tail_visited = find_tail_visited_count::<2>(&moves);
    println!("{}", tail_visited);

    let tail_visited = find_tail_visited_count::<10>(&moves);
    println!("{}", tail_visited);
}
