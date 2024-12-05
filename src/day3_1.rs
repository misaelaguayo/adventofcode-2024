use std::{fs, i32};

use regex::Regex;

pub fn ans() -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let contents =
        fs::read_to_string("inputs/input3_1").expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    let complete = lines.join("");

    let num_matches: Vec<(i32, i32)> = re
        .captures_iter(&complete)
        .map(|cap| {
            let (_, [num1, num2]) = cap.extract();
            (
                num1.to_string().parse::<i32>().unwrap(),
                num2.to_string().parse::<i32>().unwrap(),
            )
        })
        .collect();

    let products: Vec<i32> = num_matches.iter().map(|m| m.0 * m.1).collect();

    products.iter().sum::<i32>()
}
