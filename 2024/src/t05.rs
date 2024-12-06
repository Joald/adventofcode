use crate::prelude::*;

fn toposort_poor(v: &mut Vec<i64>, rules: &Vec<(i64, i64)>) {
    let mut rules = rules.clone();
    let mut i = (v.len() - 1) as i64;
    while i >= 0 {
        let (j, last) = v
            .iter()
            .find_position(|x| !rules.iter().any(|(y, _)| y == *x))
            .unwrap();
        let last = last.clone();
        println!(
            "detected {} as last, at pos={}, swapping with {} @ pos={}",
            last, j, v[i as usize], i
        );
        v.swap(i as usize, j);
        i -= 1;
        rules.retain(|(_, r)| {
            *r != last
        });

    }
}

#[allow(unused)]
pub fn solve_05(part: usize, input: String) -> i64 {
    let lines = Lines::parse(input);
    let mid = lines.iter().find_position(|s| s.is_empty()).unwrap().0;
    let mut rules = lines;
    let updates = rules.split_off(mid + 1);
    rules.pop();
    let rules = rules
        .into_iter()
        .map(|l| {
            let v: Vec<_> = l.split("|").map(|s| s.parse::<i64>().unwrap()).collect();
            (v[0], v[1])
        })
        .collect_vec();

    let sum = updates
        .iter()
        .map(|l| {
            let mut vals = l
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect_vec();
            let rules = rules
                .clone()
                .into_iter()
                .filter(|(x, y)| vals.contains(x) && vals.contains(y))
                .collect_vec();

            let s = vals
                .iter()
                .enumerate()
                .map(|(i, x)| (x, i as i64))
                .collect::<HashMap<_, _>>();

            let good = rules.iter().all(|(l, r)| s[l] < s[r]);
            if part == 2 {
                if good {
                    0
                } else {
                    //dbg!(&vals);
                    toposort_poor(&mut vals, &rules);
                    //dbg!(&vals, &rules);
                    //panic!("nope!");
                    vals[vals.len() / 2] as usize
                }
            } else if good {
                vals[vals.len() / 2] as usize
            } else {
                0
            }
        })
        .sum::<usize>() as i64;

    sum
}
