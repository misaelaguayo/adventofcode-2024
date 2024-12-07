use std::{collections::HashSet, fs, i32};

pub fn ans() -> i32 {
    let contents =
        fs::read_to_string("inputs/input6_1").expect("Should have been able to read the file");

    let grid: Vec<Vec<char>> = contents
        .lines()
        .map(|l| String::from(l).chars().collect())
        .collect();
    let r = grid.len();
    let c = grid[0].len();
    let mut pos = (0, 0);
    let mut uniqe_path: HashSet<(usize, usize)> = HashSet::new();

    let mut turns = 0;
    let turn_map: Vec<(i32, i32)> = vec![
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    for m in 0..r {
        for n in 0..c {
            if grid[m][n] == '^' {
                pos = (m, n);
                uniqe_path.insert((m, n));
            }
        }
    }

    let new_grid = grid.clone();

    loop {
        let (x, y) = pos;
        let (delta_x, delta_y) = turn_map[turns % 4];
        let new_x_i: i32 = (x as i32) + delta_x;
        let new_y_i: i32 = (y as i32) + delta_y;

        if new_x_i < 0 || new_y_i < 0 {
            break;
        }

        let new_x: usize = new_x_i as usize;
        let new_y: usize = new_y_i as usize;

        if new_x >= r || new_y >= r {
            break;
        }

        if grid[new_x][new_y] == '#' {
            turns += 1;
        } else {
            pos = (new_x, new_y);
            uniqe_path.insert((new_x, new_y));
        }
    }

    uniqe_path.len().try_into().unwrap()
}

