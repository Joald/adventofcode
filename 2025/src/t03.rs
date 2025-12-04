use std::iter::repeat_n;

use crate::prelude::*;

fn print_arrow(
    from: usize,
    to: usize,
    width: usize,
    stack: &[(usize, i64, usize)],
    suffix: String,
) {
    for i in 0..width {
        if i == from {
            print!("|");
        } else if i == to {
            if to > from {
                print!(">");
            } else {
                print!("<");
            }
        } else if to < i && i < from || from < i && i < to {
            print!("-");
        } else {
            print!(" ");
        }
    }
    println!(
        " {suffix} [{}] [from: {from}, to: {to}]",
        stack
            .iter()
            .copied()
            .map(|(_, val, _)| format!("{val}"))
            .join(",")
    );
}

#[allow(unused)]
pub fn solve_03(part: usize, input: String) -> i64 {
    let lines = Lines::parse(input);
    let mut result = 0;
    let num_of_digits = if part == 1 { 2 } else { 12 };
    for line in lines {
        let digits = line.bytes().map(|ch| (ch - b'0') as i64).collect_vec();
        if digits.is_empty() {
            continue;
        }
        println!("{}", digits.iter().map(|dig| format!("{dig}")).join(""));

        let mut suffix_counts = (0..10)
            .map(|i| repeat_n(0, digits.len()).collect_vec())
            .collect_vec();
        for (i, elem) in digits.iter().copied().enumerate().rev() {
            if i != digits.len() - 1 {
                for counts_for_digit in suffix_counts.iter_mut() {
                    counts_for_digit[i] = counts_for_digit[i + 1];
                }
            }
            suffix_counts[elem as usize][i] += 1;
        }

        let mut positions: Vec<Vec<usize>> = repeat_n(Vec::new(), 10).collect_vec();
        for (i, elem) in digits.iter().copied().enumerate().rev() {
            positions[elem as usize].push(i);
        }
        let mut stack: Vec<(usize, i64, usize)> = Vec::new();
        let mut top_digit: i64 = 9;
        let mut position = 0;
        // invariant: we can take elems from [position; len(digits))
        while stack.len() != num_of_digits {
            if top_digit < 0 {
                let (previous_position, previous_top_digit, _) = stack.pop().unwrap();
                print_arrow(
                    position,
                    previous_position,
                    digits.len(),
                    &stack,
                    format!("↺ ({top_digit} -> {previous_top_digit})"),
                );
                position = previous_position;
                top_digit = previous_top_digit - 1;
            } else if suffix_counts[top_digit as usize][position] == 0
                || positions[top_digit as usize]
                    .iter()                    
                    .all(|pos| *pos < position)
                || digits.len()
                    - positions[top_digit as usize]
                        .iter()
                        .filter(|pos| **pos >= position)
                        .next_back()
                        .unwrap()
                    < num_of_digits - stack.len()
            {
                print_arrow(
                    position,
                    position,
                    digits.len(),
                    &stack,
                    format!("⬇ ({top_digit} -> {})", top_digit - 1),
                );
                top_digit -= 1;
            } else {
                // we can still do it: push it!
                let new_position = positions[top_digit as usize]
                    .iter()
                    .filter(|pos| **pos >= position)
                    .next_back()
                    .unwrap()
                    + 1;
                stack.push((position, top_digit, new_position));
                print_arrow(
                    position,
                    new_position,
                    digits.len(),
                    &stack,
                    format!("→ ({top_digit} -> 9)"),
                );
                position = new_position;
                // but we start from 9 again as it may now fit!
                top_digit = 9;
            }
        }
        for i in 0..digits.len() {
            if stack.iter().copied().any(|(_, _, pos)| pos - 1 == i) {
                print!("^");
            } else {
                print!(" ");
            }
        }

        //println!("{}", stack.iter().fold(String::new(), |acc, (pos, _)| format!("{acc}{}^", repeat_n(" ", pos - acc.len()).join(""))));

        let line_result = stack.into_iter().fold(0, |acc, e| acc * 10 + e.1);

        println!(" = {line_result}");
        result += line_result;

        //
        //
        //let last = *digits.last().unwrap() as i64;
        //let mut line_max: i64 = 0;
        //let mut prev_max: i64 = -1;
        //let digit_count = digits.len() - 1;
        //for digit in digits.into_iter().take(digit_count) {
        //    let digit = digit as i64;
        //    line_max = vec![line_max, prev_max * 10 + digit, digit * 10 + last]
        //        .into_iter()
        //        .max()
        //        .unwrap();
        //    prev_max = cmp::max(prev_max, digit);
        //}
        //result += line_max;
    }
    result
}
