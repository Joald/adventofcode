use crate::prelude::*;

const MOD: i64 = 100;

fn md(x: i64) -> i64 {
    let fake_mod = x % MOD;
    if fake_mod < 0 {
        fake_mod + MOD
    } else {
        fake_mod
    }
}
#[allow(unused)]
pub fn solve_01(part: usize, input: String) -> i64 {
    let lines = Lines::parse(input);
    let mut pos = 50;
    let mut cnt = 0;
    for line in lines {
        let (dir, val) = line.split_at(1);
        let mut val = val.parse::<i64>().unwrap();
        for i in 0..val {
            if dir == "L" {
                pos -= 1;
            } else {
                pos += 1;
            }
            pos = md(pos);
            if part == 2 && pos == 0 {
                cnt += 1;
            }
        }
        if part == 1 && pos == 0 {
            cnt += 1;
        }
    }

    cnt
}
