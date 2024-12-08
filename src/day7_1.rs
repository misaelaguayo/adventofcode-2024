use std::{fs, i64};

fn equation_solver(ans: i64, current: i64, equation: &Vec<i64>) -> bool {
    if equation.len() == 0 {
        return current == ans;
    }

    let mul = current * equation[0];
    let add = current + equation[0];

    let new_eq = equation[1..].to_vec();
    equation_solver(ans, mul, &new_eq) || equation_solver(ans, add, &new_eq)
}

pub fn ans() -> i64 {
    let contents =
        fs::read_to_string("inputs/input7_1").expect("Should have been able to read the file");
    let mut answers = vec![];
    let mut equations = vec![];
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    for line in &lines {
        let new_line = line.replace(":", "");
        let nums: Vec<i64> = new_line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let nums_split = nums.split_first().unwrap();
        let (ans, equation_nums) = (nums_split.0.clone(), nums_split.1.to_vec().clone());

        answers.push(ans);
        equations.push(equation_nums);
    }

    let mut res = 0;

    for i in 0..answers.len() {
        if equation_solver(answers[i], equations[i][0], &equations[i][1..].to_vec()){
            res += answers[i]
        }
    }

    res
}
