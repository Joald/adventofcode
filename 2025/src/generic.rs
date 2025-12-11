use anyhow::Context;

use crate::exs::*;
use crate::t01::solve_01;
use crate::t02::solve_02;
use crate::t03::solve_03;
use crate::t04::solve_04;
use crate::t05::solve_05;
use crate::t06::solve_06;
use crate::t07::solve_07;
use crate::t08::solve_08;
use crate::t09::solve_09;
use crate::t10::solve_10;
use crate::t11::solve_11;
use crate::t12::solve_12;

pub fn read_input(task_num: usize) -> anyhow::Result<String> {
    let path = format!("tasks/{:0>2}/input.txt", task_num);
    let mut contents = std::fs::read_to_string(path.clone())
        .with_context(|| format!("Reading input from {path}"))?;
    if contents.ends_with("\n") {
        contents.pop();
    }
    Ok(contents)
}

pub fn ex_input(task_num: usize, part: usize) -> String {
    let inps = [
        vec![ex_input_1()],
        vec![ex_input_2()],
        vec![ex_input_3()],
        vec![ex_input_4()],
        vec![ex_input_5()],
        vec![ex_input_6()],
        vec![ex_input_7()],
        vec![ex_input_8()],
        vec![ex_input_9()],
        vec![ex_input_10()],
        vec![ex_input_11_1(), ex_input_11_2()],
        vec![ex_input_12()],
    ];

    let t = &inps[task_num - 1];
    let mut input = if t.len() > 1 { t[part - 1] } else { t[0] }.to_string();
    if input.ends_with("\n") {
        input.pop();
    }
    input
}

pub fn solve(task_num: usize, part: usize, input: String) -> i64 {
    let tasks = [
        solve_01, solve_02, solve_03, solve_04, solve_05, solve_06, solve_07, solve_08, solve_09,
        solve_10, solve_11, solve_12,
    ];
    tasks[task_num - 1](part, input)
}
