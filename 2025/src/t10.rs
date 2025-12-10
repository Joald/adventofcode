use itertools::repeat_n;

#[allow(unused)]
use crate::prelude::*;
#[allow(unused)]
pub fn solve_10(part: usize, input: String) -> i64 {
    let lines = Lines::parse(input).into_iter().map(|line| {
        let (diagram, rest) = line.split_once(" ").unwrap();
        let (buttons, part2) = rest.rsplit_once(" ").unwrap();
        let diagram = diagram
            .chars()
            .skip(1)
            .filter(|c| *c == '#' || *c == '.')
            .map(|c| if c == '#' { 1 } else { 0 })
            .collect::<Vec<usize>>();
        let buttons: Vec<Vec<usize>> = buttons
            .split(" ")
            .map(|item| {
                item.trim_matches(['(', ')'])
                    .split(",")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect_vec()
            })
            .collect_vec();
        (
            diagram,
            buttons,
            part2
                .trim_matches(['{', '}'])
                .split(',')
                .map(|num| num.parse::<usize>().unwrap())
                .collect_vec(),
        )
    });

    lines
        .enumerate()
        .map(|(i, (diagram, buttons, joltages))| {
            assert!(diagram.len() == joltages.len());
            println!(
                "Considering line {}: {:?} with {} buttons",
                i + 1,
                diagram,
                buttons.len()
            );
            let mut result = repeat_n(0, diagram.len()).collect_vec();
            let mut possibilities: Vec<Vec<usize>> = vec![Vec::new()];
            'outer: loop {
                let mut buf = Vec::new();
                for mut cand in std::mem::take(&mut possibilities).into_iter() {
                    cand.push(usize::MAX);
                    for i in 0..buttons.len() {
                        *cand.last_mut().unwrap() = i;
                        buf.push(cand.clone());
                    }
                }
                std::mem::swap(&mut possibilities, &mut buf);
                println!(
                    "Considering {} possibilities of len {}",
                    possibilities.len(),
                    possibilities[0].len()
                );
                for cand in possibilities.iter() {
                    for i in cand {
                        for x in buttons[*i].iter() {
                            if part == 1 {
                                result[*x] = 1 - result[*x];
                            } else {
                                result[*x] += 1;
                            }
                        }
                    }
                    result.fill(0);
                    if (part == 1 && result == diagram) || (part == 2 && result == joltages) {
                        break 'outer;
                    }
                }
            }
            let result = possibilities[0].len() as i64;
            println!("Line {} done -> {result}", i + 1);
            result
        })
        .sum()
}
