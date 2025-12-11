#[allow(unused)]
use crate::prelude::*;
#[allow(unused)]
pub fn solve_11(part: usize, input: String) -> i64 {
    let parsed_lines = Lines::parse(input);

    let lines = parsed_lines
        .iter()
        .map(|line| {
            let (from, to) = line.split_once(": ").unwrap();
            (from, to.split(" ").collect_vec())
        })
        .collect::<HashMap<_, _>>();
    dbg!(&lines);
    if part == 1 {
        let mut q = vec!["you"];
        let mut res = 0;
        while let Some(c) = q.pop() {
            for i in lines[c].iter() {
                if *i == "out" {
                    res += 1;
                } else {
                    q.push(i);
                }
            }
        }
        res
    } else {
        let mut path_cnts =
            HashMap::<&str, i64>::from_iter(lines.keys().map(|line| (*line, 0)));
        // out is not on the left in the input
        path_cnts.insert("out", 0);
        let fresh = path_cnts.clone();
        path_cnts.insert("svr", 1);
        let mut svr_to_dac = 0;
        let mut svr_to_fft = 0;
        while path_cnts.values().any(|v| *v > 0) {
            let mut next_cnts: HashMap<&str, i64> = fresh.clone();
            for (v, cnt) in path_cnts.iter() {
                if !lines.contains_key(v) {
                    continue;
                }
                for nei in lines[v].iter() {
                    *next_cnts.get_mut(nei).unwrap() += cnt;
                }
            }
            svr_to_dac += next_cnts["dac"];
            svr_to_fft += next_cnts["fft"];
            std::mem::swap(&mut path_cnts, &mut next_cnts);
        }
        let mut dac_to_fft = 0;
        let mut dac_to_out = 0;
        path_cnts.insert("dac", 1);
        while path_cnts.values().any(|v| *v > 0) {
            let mut next_cnts: HashMap<&str, i64> = fresh.clone();
            for (v, cnt) in path_cnts.iter() {
                if !lines.contains_key(v) {
                    continue;
                }
                for nei in lines[v].iter() {
                    *next_cnts.get_mut(nei).unwrap() += cnt;
                }
            }
            dac_to_out += next_cnts["out"];
            dac_to_fft += next_cnts["fft"];
            std::mem::swap(&mut path_cnts, &mut next_cnts);
        }

        let mut fft_to_dac = 0;
        let mut fft_to_out = 0;
        path_cnts.insert("fft", 1);

        while path_cnts.values().any(|v| *v > 0) {
            let mut next_cnts: HashMap<&str, i64> = fresh.clone();
            for (v, cnt) in path_cnts.iter() {
                if !lines.contains_key(v) {
                    continue;
                }
                for nei in lines[v].iter() {
                    *next_cnts.get_mut(nei).unwrap() += cnt;
                }
            }
            fft_to_dac += next_cnts["dac"];
            fft_to_out += next_cnts["out"];
            std::mem::swap(&mut path_cnts, &mut next_cnts);
        }
        svr_to_fft * fft_to_dac * dac_to_out + svr_to_dac * dac_to_fft * fft_to_out
    }
}
