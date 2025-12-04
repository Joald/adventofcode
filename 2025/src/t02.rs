use crate::prelude::*;

#[allow(unused)]
pub fn solve_02(part: usize, input: String) -> i64 {
    let ranges = input
        .trim()
        .split(",")
        .map(|range| {
            let (beg, end) = range.split_once("-").unwrap();
            (
                beg.trim().parse::<i64>().unwrap(),
                end.trim().parse::<i64>().unwrap(),
            )
        })
        .collect_vec();

    let mut cnt = 0;
    for (beg, end) in ranges {
        for i in beg..end + 1 {
            let mut pivot = 1;
            let mut lower = 0;
            let mut found = false;
            // invariant: lower = i mod pivot
            while i / (pivot * 10) > 0 {
                pivot *= 10;
                lower = i % pivot;
                let upper = i / pivot;
                let pivot_size = count_digits(pivot) - 1;
                if part == 1 {
                    if upper == lower && count_digits(lower) == pivot_size {
                        cnt += i;
                        break;
                    }
                } else {
                    let mut upper = upper;
                    let found = loop {
                        let part = upper % pivot;
                        if upper == 0 {
                            dbg!(i, upper, lower, pivot, part);
                            break true;
                        }
                        if part != lower || count_digits(part) != pivot_size {
                            break false;
                        }
                        upper /= pivot;
                    };
                    if found {
                        cnt += i;
                        break;
                    }
                }
            }
        }
    }
    cnt
}
