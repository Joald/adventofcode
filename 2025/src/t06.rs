use crate::prelude::*;
#[allow(unused)]
pub fn solve_06(part: usize, input: String) -> i64 {
    let mut lines = Lines::parse(input);
    if part == 1 {
        let symbols = lines
            .pop()
            .unwrap()
            .chars()
            .filter(|c| *c != ' ')
            .collect_vec();
        let numbers = lines
            .into_iter()
            .map(|line| {
                line.split(' ')
                    .filter_map(|chunk| chunk.parse::<i64>().ok())
                    .collect_vec()
            })
            .collect_vec();
        //dbg!(symbols, numbers, symbols.len(), numbers.);
        symbols
            .into_iter()
            .enumerate()
            .map(|(i, sym)| {
                numbers
                    .iter()
                    .fold(if sym == '+' { 0 } else { 1 }, |acc, row| {
                        if sym == '+' {
                            acc + row[i]
                        } else {
                            acc * row[i]
                        }
                    })
            })
            .sum()
    } else {
        let symbols = lines.pop().unwrap().chars().collect_vec();
        let mut current_sym = '\0';
        let mut subsum = -1;
        let lines = lines
            .into_iter()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let sum: i64 = symbols
            .into_iter()
            .enumerate()
            .map(|(i, sym)| {
                if sym != ' ' {
                    current_sym = sym;
                    match sym {
                        '+' => subsum = 0,
                        '*' => subsum = 1,
                        _ => {}
                    }
                }
                let num = combine_digits(
                    lines
                        .iter()
                        .map(|line| line[i])
                        .filter(|c| c.is_ascii_digit()),
                );
                //dbg!(current_sym, num);
                match num {
                    Some(num) => {
                        match current_sym {
                            '+' => subsum += num,
                            '*' => subsum *= num,
                            _ => {}
                        };
                        0
                    }
                    None => subsum,
                }
            })
            .sum();
        sum + subsum
    }
}
