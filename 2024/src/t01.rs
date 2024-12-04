use crate::prelude::*;

#[allow(unused)]
pub fn solve_01(part: usize, input: String) -> i64 {
    let lines = Lines::parse(input);
    dbg!(&lines);
    let (mut l, mut r): (Vec<i64>, Vec<i64>) = lines.iter().map(|l| {
        let p: Vec<i64> = l.split_whitespace().map(|x|x.parse().unwrap()).collect();
        (p[0], p[1])
    }).unzip();

    if part == 1{
        l.sort();
        r.sort();

        l.iter().zip(r).map(|(x, y)| (x-y).abs()).sum()
    } else {
        let mut rs = HashMap::new();
        for x in r {
            *rs.entry(x).or_insert(0) += 1;
        }
        l.iter().map(|k| k * rs.get(k).unwrap_or(&0)).sum()
    }    
}
