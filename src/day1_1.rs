use std::{fs, i32};

pub fn ans() -> i32 {
    let mut nums1: Vec<i32> = vec![];
    let mut nums2: Vec<i32> = vec![];
    let contents = fs::read_to_string("inputs/input1_1").expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    for line in &lines {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        nums1.push(nums[0]);
        nums2.push(nums[1]);
    }
    nums1.sort();
    nums2.sort();
    let mut res = 0;

    for i in 0..nums1.len() {
        res += (nums1[i] - nums2[i]).abs();
    }

    res
}
