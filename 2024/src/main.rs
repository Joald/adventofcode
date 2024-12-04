mod dbg;
mod exs;
mod generic;
mod t01;
mod t02;
mod t03;
mod t04;
mod t05;
mod t06;
mod t07;
mod t08;
mod t09;
mod t10;
mod t11;
mod t12;
mod t13;
mod t14;
mod t15;
mod t16;
mod t17;
mod t18;
mod t19;
mod t20;
mod t21;
mod t22;
mod t23;
mod t24;
mod t25;

use crate::generic::*;

fn var_or(var: &str, or: &str) -> usize {
    std::env::var(var)
        .unwrap_or(or.to_string())
        .parse::<usize>()
        .unwrap()
}

fn main() -> anyhow::Result<()> {
    let task_num: usize = var_or("TASK_NUM", "3");
    let part: usize = var_or("PART", "2");
    let example: bool = var_or("EXAMPLE", "0") != 0;
    println!(
        "{}",
        solve(
            task_num,
            part,
            if example {
                ex_input(task_num, part)
            } else {
                read_input(task_num)?
            }
        )
    );
    Ok(())
}
