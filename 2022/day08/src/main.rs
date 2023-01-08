const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_total_visible_trees_test() {
        assert_eq!(21, get_total_visible_trees(TEST_INPUT));
    }

    #[test]
    fn get_best_view_score() {
        let mut grid = create_grid(TEST_INPUT);
        score_grid(&mut grid);
        assert_eq!(
            8,
            grid.iter()
                .map(|i| i.iter().map(|j| j.view_score).max().unwrap())
                .max()
                .unwrap()
        );
        // dbg!(grid);
        // assert!(false);
    }
}

#[derive(Debug)]
struct Tree {
    height: i32,
    visible: bool,
    view_score: usize,
}

#[derive(PartialEq)]
enum RunGridDirection {
    Forward,
    Reverse,
}

fn create_grid(input: &str) -> Vec<Vec<Tree>> {
    let mut grid = Vec::<Vec<Tree>>::new();
    for line in input.lines() {
        let mut xaxis = Vec::<Tree>::with_capacity(line.len());

        for c in line.chars() {
            let height = c.to_digit(10).unwrap() as i32;
            xaxis.push(Tree {
                height,
                visible: false,
                view_score: 0,
            });
        }

        grid.push(xaxis);
    }
    grid
}

fn run_grid_direction_x(
    grid: &mut Vec<Vec<Tree>>,
    xlen: usize,
    ylen: usize,
    direction: RunGridDirection,
) {
    let yiterator = if direction == RunGridDirection::Forward {
        (0..ylen).collect::<Vec<usize>>()
    } else {
        (0..ylen).rev().collect::<Vec<usize>>()
    };
    for y in yiterator {
        let mut max_height_found = -1;
        for x in 0..xlen {
            max_height_found = run_grid_apply_rules(grid, x, y, max_height_found);
        }
    }
    let yiterator = if direction == RunGridDirection::Forward {
        (0..ylen).collect::<Vec<usize>>()
    } else {
        (0..ylen).rev().collect::<Vec<usize>>()
    };
    for y in yiterator {
        let mut max_height_found = -1;
        for x in (0..xlen).rev() {
            max_height_found = run_grid_apply_rules(grid, x, y, max_height_found);
        }
    }
}

fn run_grid_direction_y(
    grid: &mut Vec<Vec<Tree>>,
    xlen: usize,
    ylen: usize,
    direction: RunGridDirection,
) {
    let xiterator = if direction == RunGridDirection::Forward {
        (0..xlen).collect::<Vec<usize>>()
    } else {
        (0..xlen).rev().collect::<Vec<usize>>()
    };
    for x in xiterator {
        let mut max_height_found = -1;
        for y in 0..ylen {
            max_height_found = run_grid_apply_rules(grid, x, y, max_height_found);
        }
    }
    let xiterator = if direction == RunGridDirection::Forward {
        (0..xlen).collect::<Vec<usize>>()
    } else {
        (0..xlen).rev().collect::<Vec<usize>>()
    };
    for x in xiterator {
        let mut max_height_found = -1;
        for y in (0..ylen).rev() {
            max_height_found = run_grid_apply_rules(grid, x, y, max_height_found);
        }
    }
}

fn run_grid_apply_rules(
    grid: &mut Vec<Vec<Tree>>,
    x: usize,
    y: usize,
    max_height_found: i32,
) -> i32 {
    let height = grid[y][x].height;
    let visible = grid[y][x].visible;
    grid[y][x].visible = height > max_height_found || visible;
    std::cmp::max(height, max_height_found)
}

fn run_grid(grid: &mut Vec<Vec<Tree>>) {
    let xlen = grid[0].len();
    let ylen = grid.len();

    run_grid_direction_x(grid, xlen, ylen, RunGridDirection::Forward);
    run_grid_direction_x(grid, xlen, ylen, RunGridDirection::Reverse);
    run_grid_direction_y(grid, xlen, ylen, RunGridDirection::Forward);
    run_grid_direction_y(grid, xlen, ylen, RunGridDirection::Reverse);
}

fn count_visible_trees(grid: &Vec<Vec<Tree>>) -> usize {
    grid.iter()
        .map(|a| a.iter().filter(|t| t.visible).count())
        .sum()
}

fn get_total_visible_trees(input: &str) -> usize {
    let mut grid = create_grid(input);
    // println!("{:?}", grid);
    run_grid(&mut grid);
    // println!("{:?}", grid);
    let visible_trees_count = count_visible_trees(&grid);
    println!("{:?}", visible_trees_count);
    visible_trees_count
}

fn score_direction<FX, FY>(
    grid: &Vec<Vec<Tree>>,
    ceiling: i32,
    x: usize,
    y: usize,
    nextx: FX,
    nexty: FY,
) -> usize
where
    FX: Fn(i32) -> i32,
    FY: Fn(i32) -> i32,
{
    let mut score = 0;
    let mut max_height_found = 0i32;
    let xmax = grid[0].len() as i32;
    let ymax = grid.len() as i32;
    let mut x = nextx(x as i32);
    let mut y = nexty(y as i32);
    while max_height_found < ceiling && x >= 0i32 && x < xmax && y >= 0i32 && y < ymax {
        max_height_found = std::cmp::max(max_height_found, grid[y as usize][x as usize].height);

        score += 1;
        x = nextx(x);
        y = nexty(y);
    }
    score
}

fn score_grid_position(grid: &mut Vec<Vec<Tree>>, x: usize, y: usize) -> usize {
    let ceiling = grid[y][x].height;
    let l = score_direction(grid, ceiling, x, y, |x| x - 1, |y| y);
    let r = score_direction(grid, ceiling, x, y, |x| x + 1, |y| y);
    let u = score_direction(grid, ceiling, x, y, |x| x, |y| y - 1);
    let d = score_direction(grid, ceiling, x, y, |x| x, |y| y + 1);
    let score = l * r * u * d;
    grid[y][x].view_score = score;
    score
}

fn score_grid(grid: &mut Vec<Vec<Tree>>) {
    let xmax = grid[0].len();
    let ymax = grid.len();
    for x in 0..xmax {
        for y in 0..ymax {
            score_grid_position(grid, x, y);
        }
    }
}

fn get_highest_view_score(input: &str) -> usize {
    let mut grid = create_grid(input);
    score_grid(&mut grid);
    grid.iter()
        .map(|i| i.iter().map(|j| j.view_score).max().unwrap())
        .max()
        .unwrap()
}

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    let path = Path::new("input.txt");
    let mut file = File::open(&path).unwrap();
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();
    get_total_visible_trees(TEST_INPUT);
    get_total_visible_trees(&input);

    let highest_view_score = get_highest_view_score(&input);
    println!("{}", highest_view_score);
}
