use serde::Serialize;
use std::collections::HashMap;

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn find_first_marker_test1() {
        let input = parse(TEST_INPUT).unwrap().1;
        let mut ip = Vec::<Input>::new();
        for i in input {
            ip.push(i);
        }
        assert_eq!(Input::InputCommand(Command::ChangeDirectory("/")), ip[0]);
        assert_eq!(Input::InputCommand(Command::List), ip[1]);
        assert_eq!(Input::InputDirectory("a"), ip[2]);
        assert_eq!(Input::InputFile("b.txt", 14848514), ip[3]);
        assert_eq!(Input::InputFile("c.dat", 8504156), ip[4]);
        assert_eq!(Input::InputDirectory("d"), ip[5]);
        assert_eq!(Input::InputCommand(Command::ChangeDirectory("a")), ip[6]);
        assert_eq!(Input::InputCommand(Command::List), ip[7]);
    }
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    List,
    ChangeDirectory(&'a str),
}

#[derive(Debug, PartialEq)]
enum Input<'a> {
    InputCommand(Command<'a>),
    InputDirectory(&'a str),
    InputFile(&'a str, u64),
}

#[derive(Debug, Serialize)]
struct Directory<'a> {
    name: &'a str,
    directories: HashMap<&'a str, Directory<'a>>,
    files: Vec<File<'a>>,
    size: u64,
}
#[derive(Debug, Serialize)]
struct File<'a> {
    name: &'a str,
    size: u64,
}

impl Directory<'_> {
    fn new<'a>(directory_name: &'a str) -> Directory<'a> {
        Directory {
            name: directory_name,
            directories: HashMap::<&'a str, Directory<'a>>::new(),
            files: Vec::<File<'a>>::new(),
            size: 0u64,
        }
    }
}

fn parse_command<'a>(input: &'a str) -> nom::IResult<&'a str, Input<'a>> {
    use nom::branch::alt;
    use nom::bytes::complete::is_a;
    use nom::bytes::complete::tag;
    use nom::character::complete::char;
    use nom::sequence::preceded;
    let (rest, result) = preceded(tag("$ "), alt((tag("ls"), tag("cd"))))(input)?;

    let retval = match result {
        "ls" => Ok((rest, Input::InputCommand::<'a>(Command::List))),
        "cd" => {
            let (rest, directory) = preceded(
                char(' '),
                is_a("/abcdefghijklmnopqrstuvwxyz.ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
            )(rest)?;
            Ok((
                rest,
                Input::InputCommand(Command::ChangeDirectory(directory)),
            ))
        }
        _ => panic!(),
    };
    retval
}
fn parse_directory<'a>(input: &'a str) -> nom::IResult<&'a str, Input<'a>> {
    use nom::bytes::complete::tag;
    use nom::character::complete::alpha0;
    use nom::sequence::preceded;
    let (rest, result) = preceded(tag("dir "), alpha0)(input)?;

    Ok((rest, Input::InputDirectory(result)))
}
fn parse_file<'a>(input: &'a str) -> nom::IResult<&'a str, Input<'a>> {
    use nom::bytes::complete::is_a;
    use nom::character::complete::char;
    use nom::character::complete::digit0;
    use nom::sequence::separated_pair;
    let (rest, result) = separated_pair(
        digit0,
        char(' '),
        is_a("abcdefghijklmnopqrstuvwxyz.ABCDEFGHIJKLMNOPQRSTUVWXYZ"),
    )(input)?;

    Ok((rest, Input::InputFile(result.1, result.0.parse().unwrap())))
}
fn parse_input<'a>(input: &'a str) -> nom::IResult<&'a str, Input<'a>> {
    use nom::branch::alt;
    let (rest, result) = alt((parse_command, parse_directory, parse_file))(input)?;
    Ok((rest, result))
}

fn parse<'a>(input: &'a str) -> nom::IResult<&'a str, std::collections::VecDeque<Input<'a>>> {
    use nom::character::complete::newline;
    use nom::multi::separated_list0;

    let mut retval = std::collections::VecDeque::<Input<'a>>::new();

    let (rest, values) = separated_list0(newline, parse_input)(input)?;

    for i in values {
        retval.push_back(i);
    }

    Ok((rest, retval))
}

fn build_from_input<'a>(
    inputs: &mut std::collections::VecDeque<Input<'a>>,
    current_directory: &mut Directory<'a>,
) {
    while let Some(input) = inputs.pop_front() {
        match input {
            Input::InputDirectory(directory_name) => {
                if !current_directory.directories.contains_key(directory_name) {
                    let directory = Directory::new(directory_name);
                    current_directory
                        .directories
                        .insert(directory_name, directory);
                }
            }
            Input::InputFile(name, size) => {
                current_directory.files.push(File { name, size });
                current_directory.size += size;
            }
            Input::InputCommand(command) => match command {
                Command::ChangeDirectory(directory_name) => {
                    if directory_name != ".." {
                        let directory_option = current_directory.directories.remove(directory_name);
                        let mut directory = if directory_option.is_some() {
                            directory_option.unwrap()
                        } else {
                            Directory::new(directory_name)
                        };
                        build_from_input(inputs, &mut directory);
                        current_directory.size += directory.size;
                        current_directory
                            .directories
                            .insert(directory_name, directory);
                    } else {
                        return;
                    }
                }
                Command::List => {}
            },
            _ => panic!(),
        }
    }
}
fn build_from_input_initialise<'a>(
    input: &mut std::collections::VecDeque<Input<'a>>,
) -> anyhow::Result<Directory<'a>> {
    let Some(cmd) = input.pop_front() else { panic!("No inputs to initialise with") };
    let Input::InputCommand(Command::ChangeDirectory(init_cmd)) = cmd else { panic!("Shouldn't get here bad input.")        
    };
    let mut current_directory = Directory::new(init_cmd);
    build_from_input(input, &mut current_directory);
    let dsize = current_directory
        .directories
        .values()
        .map(|i| i.size)
        .sum::<u64>();
    let fsize = current_directory.files.iter().map(|i| i.size).sum::<u64>();
    current_directory.size = dsize + fsize;
    Ok(current_directory)
}

fn sum_directories_of_limit(directory: &Directory, limit: u64) -> u64 {
    let mut answer = directory
        .directories
        .values()
        .map(|d| sum_directories_of_limit(d, limit))
        .sum::<u64>();
    answer += if directory.size <= limit {
        directory.size
    } else {
        0
    };
    answer
}

fn find_min_to_free(directory: &Directory, need_to_free: u64) -> u64 {
    let mut to_free = u64::max_value();
    to_free = directory
        .directories
        .values()
        .map(|d| find_min_to_free(d, need_to_free))
        .min()
        .unwrap_or_else(|| to_free);
    to_free = if directory.size > need_to_free && directory.size < to_free {
        directory.size
    } else {
        to_free
    };
    to_free
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    let mut inputs = parse(&input).unwrap().1;
    // let mut inputs = parse(TEST_INPUT).unwrap().1;
    let directory = build_from_input_initialise(&mut inputs).unwrap();
    println!("{:?}", directory);
    println!("{}", serde_json::to_string_pretty(&directory).unwrap());
    let answer = sum_directories_of_limit(&directory, 100_000);
    println!("{}", answer);
    let remaining = 70_000_000 - directory.size;
    let need_to_free = 30_000_000 - remaining;

    let free_amount = find_min_to_free(&directory, need_to_free);
    println!("{}", free_amount);

    drop(inputs);
    drop(input);
}
