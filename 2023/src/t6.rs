use itertools::Itertools;

pub fn solve_6(part: usize, lines: Vec<String>) -> i64 {
    let (ts, dsts): (Vec<_>, Vec<_>) = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut line = line
                .split_once(':')
                .unwrap()
                .1
                .trim()
                .split(" ")
                .filter(|s| !s.is_empty());
            if part == 1 {
                line.map(str::parse::<i64>).map(Result::unwrap).collect()
            } else {
                vec![line.join("").parse::<i64>().unwrap()]
            }
        })
        .collect_tuple()
        .unwrap();

    ts.into_iter()
        .zip(dsts)
        .map(|(t, d)| (1..t).filter(|spent| spent * (t - spent) > d).count() as i64)
        .product()
}
