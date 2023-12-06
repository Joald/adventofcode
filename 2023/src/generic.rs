use crate::exs::*;
use crate::old::*;
use crate::t10::solve_10;
use crate::t11::solve_11;
use crate::t12::solve_12;
use crate::t13::solve_13;
use crate::t14::solve_14;
use crate::t15::solve_15;
use crate::t16::solve_16;
use crate::t17::solve_17;
use crate::t18::solve_18;
use crate::t19::solve_19;
use crate::t20::solve_20;
use crate::t21::solve_21;
use crate::t22::solve_22;
use crate::t23::solve_23;
use crate::t24::solve_24;
use crate::t25::solve_25;
use crate::t4::solve_4;
use crate::t5::solve_5;
use crate::t6::solve_6;
use crate::t7::solve_7;
use crate::t8::solve_8;
use crate::t9::solve_9;

pub fn read_input(task_num: usize) -> anyhow::Result<Vec<String>> {
    Ok(
        std::fs::read_to_string(format!("tasks/{:0>2}/input.txt", task_num))?
            .split("\n")
            .map(str::to_owned)
            .collect(),
    )
}

pub fn ex_input(task_num: usize, part: usize) -> Vec<String> {
    let inps = [
        [ex_input_1_1(), ex_input_1_2()],
        [ex_input_2(), ex_input_2()],
        [ex_input_3(), ex_input_3()],
        [ex_input_4(), ex_input_4()],
        [ex_input_5(), ex_input_5()],
        [ex_input_6(), ex_input_6()],
        [ex_input_7(), ex_input_7()],
        [ex_input_8(), ex_input_8()],
        [ex_input_9(), ex_input_9()],
        [ex_input_10(), ex_input_10()],
        [ex_input_11(), ex_input_11()],
        [ex_input_12(), ex_input_12()],
        [ex_input_13(), ex_input_13()],
        [ex_input_14(), ex_input_14()],
        [ex_input_15(), ex_input_15()],
        [ex_input_16(), ex_input_16()],
        [ex_input_17(), ex_input_17()],
        [ex_input_18(), ex_input_18()],
        [ex_input_19(), ex_input_19()],
        [ex_input_20(), ex_input_20()],
        [ex_input_21(), ex_input_21()],
        [ex_input_22(), ex_input_22()],
        [ex_input_23(), ex_input_23()],
        [ex_input_24(), ex_input_24()],
        [ex_input_25(), ex_input_25()],
    ];
    inps[task_num - 1][part - 1]
        .split("\n")
        .map(str::to_owned)
        .collect()
}

pub fn solve(task_num: usize, part: usize, lines: Vec<String>) -> i64 {
    let tasks = [
        solve_1, solve_2, solve_3, solve_4, solve_5, solve_6, solve_7, solve_8, solve_9, solve_10,
        solve_11, solve_12, solve_13, solve_14, solve_15, solve_16, solve_17, solve_18, solve_19,
        solve_20, solve_21, solve_22, solve_23, solve_24, solve_25,
    ];
    tasks[task_num - 1](part, lines)
}
