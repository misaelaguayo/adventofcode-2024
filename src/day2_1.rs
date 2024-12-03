use std::{fs, i32};

pub fn ans() -> i32 {
    let contents = fs::read_to_string("inputs/input2_1").expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(String::from).collect();

    let mut res = 0;

    for line in &lines {
        let mut is_increasing = false;
        let mut is_decreasing = false;

        let mut is_safe = true;
        let level: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        for i in 1..level.len() {
            let diff = (level[i] - level[i - 1]).abs();

            if diff == 0 || diff > 3 {
                is_safe = false;
            }

            if level[i] > level[i - 1] {
                is_increasing = true;
            }

            if level[i] < level[i - 1] {
                is_decreasing = true;
            }
        }

        if is_increasing && is_decreasing {
            is_safe = false;
        }

        if is_safe {
            res += 1;
        }
    }

    res
}
