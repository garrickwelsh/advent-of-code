#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_first_marker_test1() {
        const LINE1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(5, find_first_marker_index1(&LINE1));
    }
    #[test]
    fn find_first_marker_test2() {
        const LINE2: &str = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(6, find_first_marker_index1(&LINE2));
    }
    #[test]
    fn find_first_marker_test3() {
        const LINE3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(10, find_first_marker_index1(&LINE3));
    }
    #[test]
    fn find_first_marker_test4() {
        const LINE4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(11, find_first_marker_index1(&LINE4));
    }

    #[test]
    fn find_first_marker2_test1() {
        const LINE1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

        assert_eq!(19, find_first_marker_index2(&LINE1));
    }
    #[test]
    fn find_first_marker2_test2() {
        const LINE2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(23, find_first_marker_index2(&LINE2));
    }
    #[test]
    fn find_first_marker2_test3() {
        const LINE2: &str = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(23, find_first_marker_index2(&LINE2));
    }
    #[test]
    fn find_first_marker2_test4() {
        const LINE3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(29, find_first_marker_index2(&LINE3));
    }
    #[test]
    fn find_first_marker2_test5() {
        const LINE4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(26, find_first_marker_index2(&LINE4));
    }
}
fn find_first_marker_index(line: &str, window: usize) -> usize {
    let char_array = line.chars().collect::<Vec<char>>();

    for (i, w) in char_array.windows(window).enumerate() {
        let mut hash_map = std::collections::HashMap::<char, char>::new();
        for j in w.iter() {
            hash_map.insert(*j, *j);
        }
        if hash_map.len() == window {
            return i + window;
        }
    }
    panic!();
}

fn find_first_marker_index1(line: &str) -> usize {
    find_first_marker_index(line, 4)
}
fn find_first_marker_index2(line: &str) -> usize {
    find_first_marker_index(line, 14)
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    println!("{}", find_first_marker_index1(&input));
    println!("{}", find_first_marker_index2(&input));
}
