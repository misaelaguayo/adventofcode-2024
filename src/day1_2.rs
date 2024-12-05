use std::{collections::HashMap, fs};

pub fn ans() -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();

    let contents =
        fs::read_to_string("inputs/input1_1").expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    for line in &lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        *counts.entry(nums[1]).or_insert(0) += 1;
    }

    let mut res = 0;

    for line in &lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if counts.contains_key(&nums[0]) {
            res += counts.get(&nums[0]).unwrap() * nums[0];
        }
    }

    res
}
