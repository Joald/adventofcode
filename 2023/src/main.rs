mod dbg;
mod exs;
mod generic;
mod old;
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
mod t4;
mod t5;
mod t6;
mod t7;
mod t8;
mod t9;

use crate::generic::*;
use anyhow::Result;

fn main() -> Result<()> {
    const TASK_NUM: usize = 7;
    const PART: usize = 1;
    const EXAMPLE: bool = true;
    println!(
        "{}",
        solve(
            TASK_NUM,
            PART,
            if EXAMPLE {
                ex_input(TASK_NUM, PART)
            } else {
                read_input(TASK_NUM)?
            }
        )
    );
    Ok(())
}
