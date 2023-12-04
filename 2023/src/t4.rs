use std::collections::HashSet;
use std::iter::repeat;

fn parse_line(line: String) -> (HashSet<i64>, Vec<i64>) {
    let (_, line) = line.split_once(": ").unwrap();
    let (winning, your) = line.split_once(" | ").unwrap();
    fn parse<'a>(s: &'a str) -> impl Iterator<Item = i64> + 'a {
        s.split(" ").filter_map(|i| i.parse::<i64>().ok())
    }
    (parse(winning).collect(), parse(your).collect())
}

pub fn solve_4(part: usize, lines: Vec<String>) -> i64 {
    if part == 1 {
        lines
            .into_iter()
            .map(|line| {
                let (win, you) = parse_line(line);
                let cnt = you.iter().filter(|num| win.contains(num)).count();
                if cnt == 0 {
                    0
                } else {
                    1 << (cnt - 1)
                }
            })
            .sum()
    } else {
        let mut cnts: Vec<_> = repeat(1).take(lines.len()).collect();
        for (i, line) in lines.into_iter().enumerate() {
            let (win, you) = parse_line(line);
            let cnt = you.iter().filter(|num| win.contains(num)).count();
            for j in i + 1..i + 1 + cnt {
                cnts[j] += cnts[i];
            }
        }
        cnts.iter().sum()
    }
}
