use std::{fs, i32};

fn get_diagonal(grid: &Vec<Vec<char>>, m: usize, n: usize) -> i32 {
    let mut left_diagonal = "".to_owned();
    let mut right_diagonal = "".to_owned();

    left_diagonal.push(grid[m - 1][n - 1]);
    left_diagonal.push(grid[m][n]);
    left_diagonal.push(grid[m + 1][n + 1]);

    right_diagonal.push(grid[m - 1][n + 1]);
    right_diagonal.push(grid[m][n]);
    right_diagonal.push(grid[m + 1][n - 1]);

    if (left_diagonal != "MAS" && left_diagonal != "SAM")
        || (right_diagonal != "MAS" && right_diagonal != "SAM")
    {
        return 0;
    }

    return 1;
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

    for i in 1..r - 1 {
        for j in 1..c - 1 {
            res += get_diagonal(&char_grid, i, j)
        }
    }

    res
}
