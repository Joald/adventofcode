use crate::exs::*;
use crate::old::*;

pub fn read_input(task_num: usize) -> anyhow::Result<Vec<String>> {
    Ok(
        std::fs::read_to_string(format!("tasks/{:0>2}/input.txt", task_num))?
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect(),
    )
}

pub fn ex_input(task_num: usize, part: usize) -> Vec<String> {
    let inps = [
        [ex_input_1_1(), ex_input_1_2()],
        [ex_input_2(), ex_input_2()],
        [ex_input_3(), ex_input_3()],
    ];
    inps[task_num - 1][part - 1]
        .split("\n")
        .map(|s| s.to_owned())
        .collect()
}

pub fn solve(task_num: usize, part: usize, lines: Vec<String>) -> i64 {
    let tasks = [solve_1, solve_2, solve_3];
    tasks[task_num - 1](part, lines)
}
