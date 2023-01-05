use itertools::Itertools;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_first_marker_test1() {
        const LINE1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

        assert_eq!(5 - 1, find_first_marker_index(&LINE1));
    }
    #[test]
    fn find_first_marker_test2() {
        const LINE2: &str = "nppdvjthqldpwncqszvftbrmjlhg";

        assert_eq!(6 - 1, find_first_marker_index(&LINE2));
    }
    #[test]
    fn find_first_marker_test3() {
        const LINE3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";

        assert_eq!(10 - 1, find_first_marker_index(&LINE3));
    }
    #[test]
    fn find_first_marker_test4() {
        const LINE4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

        assert_eq!(11 - 1, find_first_marker_index(&LINE4));
    }
}
fn find_first_marker_index(line: &str) -> usize {
    let mut list = std::collections::LinkedList::new();
    for (i, c) in line.char_indices() {
        if list.len() == 4 {
            list.pop_back();
        }
        list.push_front(c);
        if list.len() == 4 {
            if list.iter().all_unique() {
                return i;
            }
        }
    }
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
    println!("{}", find_first_marker_index(&input) + 1);
}
