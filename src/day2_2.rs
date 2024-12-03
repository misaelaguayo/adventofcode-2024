use std::{fs, i32};

pub fn level_is_safe(levels: Vec<i32>) -> bool {
    let mut is_safe = true;

    let mut is_increasing = false;
    let mut is_decreasing = false;

    for i in 1..levels.len() {
        let diff = (levels[i] - levels[i - 1]).abs();

        if diff == 0 || diff > 3 {
            is_safe = false;
        }

        if levels[i] > levels[i - 1] {
            is_increasing = true;
        }

        if levels[i] < levels[i - 1] {
            is_decreasing = true;
        }

        if is_increasing && is_decreasing {
            is_safe = false;
        }
    }

    is_safe
}

pub fn ans() -> i32 {
    let contents = fs::read_to_string("input2_1").expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(String::from).collect();

    let mut res = 0;

    for line in &lines {
        let level: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut is_safe = false;
        for i in 0..level.len() {
            let new_level: Vec<i32> = level
                .iter()
                .enumerate()
                .filter_map(
                    |(index, &value)| {
                        if index != i {
                            Some(value)
                        } else {
                            None
                        }
                    },
                )
                .collect();

            if level_is_safe(new_level) {
                is_safe = true;
            }
        }

        if is_safe {
            res += 1;
        }
    }

    res
}
