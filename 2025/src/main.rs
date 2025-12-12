mod dbg;
mod exs;
mod generic;
mod prelude;
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

use crate::generic::*;

use aoc_common::var_or;

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
