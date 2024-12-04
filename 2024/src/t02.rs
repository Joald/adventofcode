use itertools::Itertools;

fn gen_missings(mut v: Vec<i64>) -> Vec<Vec<i64>> {
    let mut result = Vec::new();
    result.push(v.clone());
    let mut rmd = v.pop().unwrap();
    for i in (0..v.len()).rev() {
        result.push(v.clone());
        std::mem::swap(&mut rmd, v.get_mut(i).unwrap());
    }
    result.push(v);
    //dbg!(&result);
    result
}

#[allow(unused)]
pub fn solve_02(part: usize, lines: Vec<String>) -> i64 {
    //dbg!(&lines);
    let v: Vec<Vec<_>> = lines
        .iter()
        .map(|l| {
            l.split_whitespace()
                .map(|tok| tok.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    v.into_iter()
        .map(|report_orig| {
            if part == 1 {
                vec![report_orig.clone()]
            } else {
                gen_missings(report_orig)
            }
            .iter()
            .any(|report| {
                ((report.is_sorted() || report.is_sorted_by_key(|x| -x))
                    && report.iter().zip(report.iter().dropping(1)).all(|(l, r)| {
                        let diff = (l - r).abs();
                        (1..=3).contains(&diff)
                    }))
            }) as i64
        })
        .sum()
}
