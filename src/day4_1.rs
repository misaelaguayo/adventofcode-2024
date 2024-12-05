use std::{fs, i32};

fn left_up_diagonal(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    let mut word = "".to_owned();

    if n < 3 || m < 3 {
        return 0;
    }

    for i in 0..4 {
        word.push(grid[m - i][n - i]);
    }

    if word == "XMAS" || word == "SAMX" {
        return 1;
    }

    return 0;
}

fn right_up_diagonal(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    let mut word = "".to_owned();
    let c = grid[0].len();

    if m < 3 || c - n < 4 {
        return 0;
    }

    for i in 0..4 {
        word.push(grid[m - i][n + i]);
    }

    if word == "XMAS" || word == "SAMX" {
        return 1;
    }

    return 0;
}

fn left_down_diagonal(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    let r = grid.len();
    let mut word = "".to_owned();

    if n < 3 || r - m < 4 {
        return 0;
    }

    for i in 0..4 {
        word.push(grid[m + i][n - i]);
    }

    if word == "XMAS" || word == "SAMX" {
        return 1;
    }

    return 0;
}

fn right_down_diagonal(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    let r = grid.len();
    let c = grid[0].len();
    let mut word = "".to_owned();

    if c - n < 4 || r - m < 4 {
        return 0;
    }

    for i in 0..4 {
        word.push(grid[m + i][n + i]);
    }

    if word == "XMAS" || word == "SAMX" {
        return 1;
    }

    return 0;
}

fn right(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    let c = grid[0].len();

    if c - n < 4 {
        return 0;
    }

    let mut word = "".to_owned();

    for i in 0..4 {
        word.push(grid[m][n + i]);
    }

    if word == "XMAS" || word == "SAMX" {
        return 1;
    }

    return 0;
}

fn left(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    if n < 3 {
        return 0;
    }

    let mut word = "".to_owned();

    for i in 0..4 {
        word.push(grid[m][n - i]);
    }

    if word == "XMAS" || word == "SAMX" {
        return 1;
    }

    return 0;
}

fn down(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    let r = grid.len();

    if r - m < 4 {
        return 0;
    }

    let mut word = "".to_owned();

    for i in 0..4 {
        word.push(grid[m + i][n]);
    }

    if word == "XMAS" || word == "SAMX" {
        return 1;
    }

    return 0;
}

fn up(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    if m < 3 {
        return 0;
    }

    let mut word = "".to_owned();

    for i in 0..4 {
        word.push(grid[m - i][n]);
    }

    if word == "XMAS" || word == "SAMX" {
        return 1;
    }

    return 0;
}

pub fn ans() -> i32 {
    let contents =
        fs::read_to_string("inputs/input4_1").expect("Should have been able to read the file");
    let char_grid: Vec<Vec<char>> = contents
        .lines()
        .map(|s| String::from(s).chars().collect())
        .collect();
    let r = char_grid.len();
    let c = char_grid[0].len();

    let mut res = 0;

    for i in 0..r {
        for j in 0..c {
            res += right(&char_grid, i, j);
            res += left(&char_grid, i, j);
            res += up(&char_grid, i, j);
            res += down(&char_grid, i, j);
            res += left_up_diagonal(&char_grid, i, j);
            res += right_up_diagonal(&char_grid, i, j);
            res += left_down_diagonal(&char_grid, i, j);
            res += right_down_diagonal(&char_grid, i, j);
        }
    }

    res / 2
}
