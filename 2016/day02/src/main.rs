use std::str::Chars;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn given_keypad1_moves_then() {
        const LINE1: &str = "ULL";
        const LINE2: &str = "RRDDD";
        const LINE3: &str = "LURDL";
        const LINE4: &str = "UUUUD";

        given_keypad1_moves_with_starting_location_then(LINE1.chars(), 1, 1, '1');
        given_keypad1_moves_with_starting_location_then(LINE2.chars(), 0, 0, '9');
        given_keypad1_moves_with_starting_location_then(LINE3.chars(), 2, 2, '8');
        given_keypad1_moves_with_starting_location_then(LINE4.chars(), 1, 2, '5');
    }

    #[test]
    fn given_keypad2_moves_then() {
        const LINE1: &str = "ULL";
        const LINE2: &str = "RRDDD";
        const LINE3: &str = "LURDL";
        const LINE4: &str = "UUUUD";

        given_keypad2_moves_with_starting_location(LINE1.chars(), 0, 2, '5');
        given_keypad2_moves_with_starting_location(LINE2.chars(), 0, 2, 'D');
        given_keypad2_moves_with_starting_location(LINE3.chars(), 2, 4, 'B');
        given_keypad2_moves_with_starting_location(LINE4.chars(), 2, 3, '3');
    }

    fn given_keypad2_moves_with_starting_location(
        moves: Chars,
        x: usize,
        y: usize,
        expected_keypad_value: char,
    ) {
        assert_eq!(
            expected_keypad_value,
            calculate2_code_with_starting_location(moves, x, y).keypad_value
        );
    }

    fn given_keypad1_moves_with_starting_location_then(
        moves: Chars,
        x: usize,
        y: usize,
        expected_keypad_value: char,
    ) {
        assert_eq!(
            expected_keypad_value,
            calculate_code_with_starting_location(moves, x, y).keypad_value
        );
    }
}

const KEYPAD: [[char; 3]; 3] = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

const KEYPAD2: [[Option<char>; 5]; 5] = [
    [None, None, Some('1'), None, None],
    [None, Some('2'), Some('3'), Some('4'), None],
    [Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    [None, Some('A'), Some('B'), Some('C'), None],
    [None, None, Some('D'), None, None],
];

struct Keypad2Result {
    x: usize,
    y: usize,
    keypad_value: char,
}

fn calculate2_code_with_starting_location(moves: Chars, x: usize, y: usize) -> Keypad2Result {
    let (mut x, mut y) = (x, y);

    for c in moves {
        match c {
            'U' => {
                y = if y == 0 {
                    0
                } else if KEYPAD2[y - 1][x].is_none() {
                    y
                } else {
                    y - 1
                }
            }
            'D' => {
                y = if y == 4 {
                    4
                } else if KEYPAD2[y + 1][x].is_none() {
                    y
                } else {
                    y + 1
                }
            }
            'L' => {
                x = if x == 0 {
                    0
                } else if KEYPAD2[y][x - 1].is_none() {
                    x
                } else {
                    x - 1
                }
            }
            'R' => {
                x = if x == 4 {
                    4
                } else if KEYPAD2[y][x + 1].is_none() {
                    x
                } else {
                    x + 1
                }
            }
            _ => panic!(),
        };
    }
    Keypad2Result {
        x,
        y,
        keypad_value: KEYPAD2[y][x].unwrap(),
    }
}

fn calculate_code_with_starting_location(moves: Chars, x: usize, y: usize) -> Keypad2Result {
    let (mut x, mut y): (usize, usize) = (x, y);
    for c in moves {
        match c {
            'U' => y = if y == 0 { 0 } else { y - 1 },
            'D' => y = if y == 2 { 2 } else { y + 1 },
            'L' => x = if x == 0 { 0 } else { x - 1 },
            'R' => x = if x == 2 { 2 } else { x + 1 },
            _ => panic!(),
        };
    }
    Keypad2Result {
        x,
        y,
        keypad_value: KEYPAD[y][x],
    }
}

fn main() {
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;
    use std::path::Path;

    let path = Path::new("input.txt");
    let file = File::open(&path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut previous_keypad_result = Keypad2Result {
        x: 1usize,
        y: 1usize,
        keypad_value: ' ',
    };
    let mut output = String::new();
    for line in lines {
        let line = line.unwrap();
        let keypad_result = calculate_code_with_starting_location(
            line.chars(),
            previous_keypad_result.x,
            previous_keypad_result.y,
        );
        output.push(keypad_result.keypad_value);
        previous_keypad_result = keypad_result;
    }
    println!("{}", output);

    let file = File::open(&path).unwrap();
    let lines = io::BufReader::new(file).lines();
    let mut previous_keypad_result = Keypad2Result {
        x: 0usize,
        y: 2usize,
        keypad_value: ' ',
    };
    let mut output = String::new();
    for line in lines {
        let line = line.unwrap();
        let keypad_result = calculate2_code_with_starting_location(
            line.chars(),
            previous_keypad_result.x,
            previous_keypad_result.y,
        );
        output.push(keypad_result.keypad_value);
        previous_keypad_result = keypad_result;
    }
    println!("{}", output);
}
