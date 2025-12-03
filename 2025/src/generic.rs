use std::collections::HashMap;

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

pub fn read_input(task_num: usize) -> anyhow::Result<String> {
    let mut contents = std::fs::read_to_string(format!("tasks/{:0>2}/input.txt", task_num))?;
    if contents.ends_with("\n") {
        contents.pop();
    }
    Ok(contents)
}

pub fn ex_input(task_num: usize, part: usize) -> String {
    let inps = [
        vec![ex_input_1()],
        vec![ex_input_2()],
        vec![ex_input_3_1(), ex_input_3_2()],
        vec![ex_input_4()],
        vec![ex_input_5()],
        vec![ex_input_6()],
        vec![ex_input_7()],
        vec![ex_input_8()],
        vec![ex_input_9()],
        vec![ex_input_10()],
        vec![ex_input_11()],
        vec![ex_input_12()],
        vec![ex_input_13()],
        vec![ex_input_14()],
        vec![ex_input_15()],
        vec![ex_input_16()],
        vec![ex_input_17()],
        vec![ex_input_18()],
        vec![ex_input_19()],
        vec![ex_input_20()],
        vec![ex_input_21()],
        vec![ex_input_22()],
        vec![ex_input_23()],
        vec![ex_input_24()],
        vec![ex_input_25()],
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
        solve_10, solve_11, solve_12, solve_13, solve_14, solve_15, solve_16, solve_17, solve_18,
        solve_19, solve_20, solve_21, solve_22, solve_23, solve_24, solve_25,
    ];
    tasks[task_num - 1](part, input)
}

pub trait InputParser {
    type Res;
    fn parse(input: String) -> Self::Res;
}

pub struct Lines {}
impl InputParser for Lines {
    type Res = Vec<String>;
    fn parse(input: String) -> Vec<String> {
        input.split("\n").map(str::to_owned).collect()
    }
}

pub type CoordsResult = HashMap<i64, HashMap<i64, char>>;
pub struct Coords {}
impl InputParser for Coords {
    type Res = CoordsResult;
    fn parse(input: String) -> Self::Res {
        Lines::parse(input)
            .iter()
            .enumerate()
            .map(|(x, l)| {
                let row = l.chars().enumerate().map(|(y, c)| (y as i64, c)).collect();
                (x as i64, row)
            })
            .collect()
    }
}

