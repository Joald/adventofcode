use itertools::Itertools;
use std::collections::HashMap;

#[allow(unused)]
pub fn solve_04(part: usize, lines: Vec<String>) -> i64 {
    let crds: HashMap<i64, HashMap<i64, char>> = lines
        .iter()
        .enumerate()
        .map(|(x, l)| {
            let row = l.chars().enumerate().map(|(y, c)| (y as i64, c)).collect();
            (x as i64, row)
        })
        .collect();

    let w = lines[0].len() as i64;
    let h = lines.len() as i64;
    if part == 1 {
        let offs = vec![
            (0, 1),
            (0, -1),
            (-1, 0),
            (1, 0),
            (-1, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
        ];

        let pat: Vec<_> = "XMAS".chars().collect();
        let mut sum = 0;

        for (i, j) in (0..h).cartesian_product(0..w) {
            for (offi, offj) in &offs {
                let mut all_good = true;
                for k in 0..4 {
                    let x = i + offi * k;
                    let y = j + offj * k;
                    if !(0..h).contains(&x)
                        || !(0..w).contains(&y)
                        || crds[&x][&y] != pat[k as usize]
                    {
                        all_good = false;
                        break;
                    }
                }
                if all_good {
                    sum += 1;
                }
            }
        }
        sum
    } else {
        let offs = vec![
            (1, 1),
            (-1, 1)
        ];

        let mut sum = 0;
        for (i, j) in (0..h).cartesian_product(0..w) {
            if crds[&i][&j] != 'A' {
                continue;
            }
            let mut all_good = true;
            for (offi, offj) in &offs {
                let x1 = i + offi;
                let x2 = i - offi;
                let y1 = j + offj;
                let y2 = j - offj;

                if !(0..h).contains(&x1) || !(0..h).contains(&x2) || !(0..w).contains(&y1)|| !(0..w).contains(&y2) {
                    all_good = false;
                    break;
                }
                let c1 = crds[&x1][&y1];
                let c2 = crds[&x2][&y2];
                if c1 == 'M' && c2 == 'S' || c1 == 'S' && c2 == 'M' {
                    continue;
                }
                all_good = false;
                break;
            }
            if all_good {
                sum += 1;
            }
        }
        sum
    }
}
