mod dbg;
mod exs;
mod generic;
mod old;

use crate::generic::*;
use anyhow::Result;

fn main() -> Result<()> {
    const TASK_NUM: usize = 3;
    const PART: usize = 2;
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
