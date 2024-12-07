use std::{collections::HashMap, fs, i32};

pub fn ans() -> i32 {
    let contents =
        fs::read_to_string("inputs/input5_1").expect("Should have been able to read the file");
    let lines: Vec<String> = contents.lines().map(String::from).collect();

    let mut before_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut is_update = false;
    let mut all_updates = vec![];
    let mut valid_updates = vec![];

    for line in &lines {
        // println!("{line}");
        if line.len() == 0 {
            is_update = true;
        }

        if !is_update {
            let rules: Vec<i32> = line.split("|").map(|s| s.parse::<i32>().unwrap()).collect();

            let k = rules[0];
            let v = rules[1];

            before_map.entry(k).or_insert_with(Vec::new).push(v);
        } else {
            if line.len() == 0 {
                continue;
            }
            let update: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

            all_updates.push(update);
        }
    }

    for row in &all_updates {
        let mut is_valid_update = true;
        for i in 0..row.len() {
            let nums_before = before_map.get(&row[i]);

            match nums_before {
                Some(num) => {
                    for j in 0..i {
                        if num.contains(&row[j]) {
                            is_valid_update = false;
                        }
                    }
                }
                None => {
                    is_valid_update = true;
                }
            }
        }

        if is_valid_update {
            valid_updates.push(row);
        }
    }

    let mut res = 0;

    for valid_update in &valid_updates {
        let middle = valid_update[valid_update.len() / 2];
        res += middle;
    }

    res
}
