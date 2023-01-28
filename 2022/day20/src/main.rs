use std::collections::VecDeque;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn parse_data_test() {
        let (remaining, data) = parse_input(TEST_INPUT).unwrap();
        assert_eq!("", remaining);
        assert_eq!(1, data[0].value);
    }

    #[test]
    #[ignore]
    fn perform_moves_test() {
        let (_, mut data) = parse_input(TEST_INPUT).unwrap();
        perform_moves(&mut data);
        assert_eq!(1, data[0].value);
        assert_eq!(2, data[1].value);
        assert_eq!(-3, data[2].value);
        assert_eq!(4, data[3].value);
        assert_eq!(0, data[4].value);
        assert_eq!(3, data[5].value);
        assert_eq!(-2, data[6].value);
    }

    #[test]
    fn analyse_moves_test() {
        const MOVE1: [i64; 7] = [2, 1, -3, 3, -2, 0, 4];
        let (_, mut data) = parse_input(TEST_INPUT).unwrap();
        perform_move(&mut data, 0);
        data.make_contiguous();
        assert_eq!(
            MOVE1,
            data.iter()
                .map(|d| d.value)
                .collect::<Vec<i64>>()
                .as_slice()
        );
        const MOVE2: [i64; 7] = [1, -3, 2, 3, -2, 0, 4];
        perform_move(&mut data, 1);
        data.make_contiguous();
        assert_eq!(
            MOVE2,
            data.iter()
                .map(|d| d.value)
                .collect::<Vec<i64>>()
                .as_slice()
        );
        const MOVE3: [i64; 7] = [1, 2, 3, -2, -3, 0, 4];
        perform_move(&mut data, 2);
        data.make_contiguous();
        assert_eq!(
            MOVE3,
            data.iter()
                .map(|d| d.value)
                .collect::<Vec<i64>>()
                .as_slice()
        );
        const MOVE4: [i64; 7] = [1, 2, -2, -3, 0, 3, 4];
        perform_move(&mut data, 3);
        data.make_contiguous();
        assert_eq!(
            MOVE4,
            data.iter()
                .map(|d| d.value)
                .collect::<Vec<i64>>()
                .as_slice()
        );
        const MOVE5: [i64; 7] = [1, 2, -3, 0, 3, 4, -2];
        perform_move(&mut data, 4);
        data.make_contiguous();
        assert_eq!(
            MOVE5,
            data.iter()
                .map(|d| d.value)
                .collect::<Vec<i64>>()
                .as_slice()
        );
        const MOVE6: [i64; 7] = [1, 2, -3, 0, 3, 4, -2];
        perform_move(&mut data, 5);
        data.make_contiguous();
        assert_eq!(
            MOVE6,
            data.iter()
                .map(|d| d.value)
                .collect::<Vec<i64>>()
                .as_slice()
        );
        const MOVE7: [i64; 7] = [1, 2, -3, 4, 0, 3, -2];
        perform_move(&mut data, 6);
        data.make_contiguous();
        assert_eq!(
            MOVE7,
            data.iter()
                .map(|d| d.value)
                .collect::<Vec<i64>>()
                .as_slice()
        );
    }

    #[test]
    #[ignore]
    fn get_grove_coordinates_test() {
        let (_, mut data) = parse_input(TEST_INPUT).unwrap();
        perform_moves(&mut data);
        let (x, y, z) = get_grove_coordinates(&data);
        assert_eq!(4, x);
        assert_eq!(-3, y);
        assert_eq!(2, z);
    }
    #[test]
    #[ignore]
    fn get_sum_grove_coordinates_test() {
        let (_, mut data) = parse_input(TEST_INPUT).unwrap();
        perform_moves(&mut data);
        let s = get_sum_of_grove_coordinates(&data);
        assert_eq!(3, s);
    }
    #[test]
    fn analyse_moves_part2_test() {
        const MOVE1: [i64; 7] = [
            0,
            -2434767459,
            3246356612,
            -1623178306,
            2434767459,
            1623178306,
            811589153,
        ];
        let (_, data) = parse_input(TEST_INPUT).unwrap();
        let mut data = translate_to_part2_input(&data);
        // perform_moves_part2(&mut data);
        perform_moves(&mut data);
        assert_eq!(
            MOVE1,
            data.iter()
                .map(|d| d.value)
                .collect::<Vec<i64>>()
                .as_slice()
        );
    }
}

const TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

#[derive(Debug, Clone)]
struct Data {
    original_position: i64,
    value: i64,
}

fn get_sum_of_grove_coordinates(data: &VecDeque<Data>) -> i64 {
    let (x, y, z) = get_grove_coordinates(&data);
    x + y + z
}

fn get_grove_coordinates(data: &VecDeque<Data>) -> (i64, i64, i64) {
    let len = data.len();
    let idx = data.iter().position(|d| d.value == 0).unwrap();
    (
        data[(idx + 1000) % len].value,
        data[(idx + 2000) % len].value,
        data[(idx + 3000) % len].value,
    )
}

fn perform_moves_part2(data: &mut VecDeque<Data>) {
    for _ in 0..10 {
        perform_moves(data);
    }
}

fn perform_moves(data: &mut VecDeque<Data>) {
    let len = data.len() as i64;
    for i in 0..len {
        perform_move(data, i);
    }
}

fn perform_move(data: &mut VecDeque<Data>, i: i64) {
    let len = data.len() as i64;
    let from = data.iter().position(|d| d.original_position == i).unwrap();
    let d = data.remove(from).unwrap();
    let from = from as i64;
    let raw_to = from + d.value % (len - 1);

    let to = if d.value == 0 {
        from
    } else if d.value > 0 {
        let t = raw_to % len;
        if t < from {
            t + 1
        } else {
            t
        }
    } else {
        let mut t = raw_to % len;
        if t <= 0 {
            t -= 1
        }
        if t < 0 {
            t = len + t;
        }
        t
    };
    // let mut to = if d.value > 0 {
    //     if raw_to < from {
    //         raw_to + 1
    //     } else {
    //         raw_to
    //     }
    // } else if d.value == 0 {
    //     from
    // } else {
    //     if raw_to > from {
    //         len + raw_to - 1
    //     } else {
    //         len + raw_to
    //     }
    // };
    // if d.value > 0 && to < from {
    //     to += 1;
    // }
    println!(
        "orig - {}, from - {} raw_to - {} to - {}, value - {}",
        i, from, raw_to, to, d.value
    );
    data.insert(to as usize, d);
}

fn parse_input<'a>(input: &'a str) -> nom::IResult<&'a str, VecDeque<Data>> {
    use nom::character::complete::{i64, newline};
    use nom::multi::separated_list0;

    let (remaining, data) = separated_list0(newline, i64)(input)?;
    Ok((
        remaining,
        data.into_iter()
            .enumerate()
            .map(|(i, v)| Data {
                original_position: i as i64,
                value: v,
            })
            .collect::<VecDeque<Data>>(),
    ))
}

fn translate_to_part2_input(data: &VecDeque<Data>) -> VecDeque<Data> {
    data.iter()
        .map(|d| Data {
            original_position: d.original_position,
            value: d.value * 811589153,
        })
        .collect::<VecDeque<Data>>()
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    let trimmed_input = input.trim();
    let (_, mut data) = parse_input(&trimmed_input).unwrap();
    perform_moves(&mut data);
    let s = get_sum_of_grove_coordinates(&data);
    println!("{}", s);

    let (_, data) = parse_input(&trimmed_input).unwrap();
    let mut data = translate_to_part2_input(&data);
    perform_moves_part2(&mut data);
    let s = get_sum_of_grove_coordinates(&data);
    println!("{}", s);
}
