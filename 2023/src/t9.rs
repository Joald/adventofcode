use itertools::Itertools;

#[allow(unused)]
pub fn solve_9(part: usize, lines: Vec<String>) -> i64 {
    let hists: Vec<Vec<i64>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(' ')
                .map(str::parse::<i64>)
                .map(Result::unwrap)
                .collect()
        })
        .collect();
    hists
        .into_iter()
        .map(|hist| {
            let mut ders = Vec::from([hist]);
            while !ders.last().unwrap().iter().all_equal() {
                let prev = ders.last().unwrap();
                ders.push(
                    (0..prev.len() - 1)
                        .zip(1..prev.len())
                        .map(|(l, r)| prev[r] - prev[l])
                        .collect(),
                )
            }
            {
                let consts = ders.last_mut().unwrap();
                consts.push(*consts.last().unwrap());
            }
            for vi in (0..ders.len() - 1).rev() {
                let val = ders[vi + 1].last().unwrap() + ders[vi].last().unwrap();
                ders[vi].push(val);
            }

            if part == 1 {
                *ders[0].last().unwrap()
            } else {
                ders.iter()
                    .rev()
                    .fold(0, |acc, x| *x.first().unwrap() - acc)
            }
        })
        .sum()
}
