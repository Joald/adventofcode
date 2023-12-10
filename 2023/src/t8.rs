use crate::dbg::dprintln;
use std::collections::{HashMap, HashSet};
use std::{panic, usize};

#[allow(unused)]
pub fn solve_8(part: usize, lines: Vec<String>) -> i64 {
    let dirs: Vec<_> = lines.first().unwrap().chars().collect();
    let mut graph = HashMap::new();
    for line in lines.iter().skip(2) {
        if line.is_empty() {
            continue;
        }
        let (src, rest) = line.split_once(" = (").unwrap();
        let (l, r) = rest.split_once(", ").unwrap();
        let r = &r[0..3];
        graph.entry(src).or_insert((l, r));
    }

    let go = |v, c| {
        let (l, r) = graph.get(v).unwrap();
        if c == 'L' {
            *l
        } else {
            *r
        }
    };
    if part == 1 {
        let mut curs = vec!["AAA"];
        let mut i = 0;
        let mut res = 0;
        while !curs.iter().all(|cur| cur.ends_with('Z')) {
            curs = curs
                .into_iter()
                .map(|cur| {
                    let cur = go(cur, dirs[i]);

                    cur
                })
                .collect();

            res += 1;
            i = (i + 1) % dirs.len();
        }

        res
    } else {
        // screw this
        return part2(dirs, graph);
        let mut ss: Vec<_> = graph
            .keys()
            .map(|x| *x)
            .filter(|k| k.ends_with('A'))
            .collect();
        // let mut vis: HashMap<&str, Vec<&str>> = ss.iter().map(|x| (x, vec![x])).collect();
        let mut cycles: HashMap<&str, (i64, i64)> = HashMap::new();
        for starter in ss.iter() {
            let starter = *starter;
            let mut i = 0;
            let mut cur = starter;
            let mut total = 1;
            let mut vis = HashMap::from([((cur, i), total)]);
            loop {
                cur = go(cur, dirs[i]);
                i = (i + 1) % dirs.len();
                total += 1;
                let last_seen = vis.entry((cur, i)).or_insert(total);
                if *last_seen != total {
                    cycles.insert(starter, (total, *last_seen));
                    break;
                }
            }
        }
        let full_start = cycles.values().map(|(_, v)| *v).max().unwrap();
        dprintln!("full_start={}", full_start);
        let fsi = full_start % dirs.len() as i64;
        let ns: HashMap<_, _> = ss
            .iter()
            .map(|starter| {
                let mut cur = *starter;
                for i in 0..full_start {
                    cur = go(cur, dirs[i as usize % dirs.len()]);
                }
                (*starter, cur)
            })
            .collect();
        dprintln!("ns={:?}", ns);

        let winners: HashMap<&str, Vec<(i64, &str)>> = ns
            .iter()
            .map(|(start, new_start)| {
                let mut cur = *new_start;
                let mut wins = Vec::new();

                for i in 0..=cycles[start].0 - cycles[start].1 {
                    if cur.ends_with('Z') {
                        wins.push((i + 1, cur));
                    }
                    cur = go(cur, dirs[(full_start + i) as usize % dirs.len()])
                }

                (*start, wins)
            })
            .collect();

        dprintln!("cycles: {:?},\nwinners: {:?}", cycles, winners);

        let mut is: Vec<(&&str, usize)> = ss.iter().zip(std::iter::repeat(0)).collect();
        let mut mini: i64 = i64::MAX;

        loop {
            let mut res = 1;
            for (v, i) in is.iter() {
                let val = (winners[**v])[*i].0;
                dprintln!("v={}, i={}, val={}", v, i, val);
                let result = panic::catch_unwind(|| num::integer::lcm(res, val));
                if result.is_err() {
                    break;
                }
                res = result.unwrap();
            }
            dprintln!("cand is {:?}", res);
            mini = mini.min(res);
            let mut ended = false;
            for mut i in is.iter_mut() {
                i.1 += 1;
                if i.1 == winners[i.0].len() {
                    i.1 = 0;
                } else {
                    ended = true;
                    break;
                }
            }
            if !ended {
                break;
            }
        }

        full_start + mini
    }
}

fn part2(dirs: Vec<char>, graph: HashMap<&str, (&str, &str)>) -> i64 {
    let go = |v, c| {
        let (l, r) = graph.get(v).unwrap();
        if c == 'L' {
            *l
        } else {
            *r
        }
    };
    let vs: Vec<&str> = graph.keys().map(|x| *x).collect();
    let starts: Vec<&str> = vs
        .clone()
        .into_iter()
        .filter(|k| k.ends_with('A'))
        .collect();
    let ends: HashSet<_> = vs
        .clone()
        .into_iter()
        .filter(|k| k.ends_with('Z'))
        .collect();

    let mut max_tail = 0;

    let mut paths: HashMap<_, _> = starts
        .clone()
        .into_iter()
        .zip(starts.clone().into_iter().map(|start| {
            let mut cur = start;
            let mut vis = HashMap::new();
            let mut path = Vec::new();
            for (i, total) in (0..dirs.len()).cycle().zip(0usize..) {
                let last_seen = vis.entry((cur, i)).or_insert(total);
                if *last_seen != total {
                    max_tail = max_tail.max(*last_seen);
                    return (*last_seen, path);
                }
                path.push(cur);
                cur = go(cur, dirs[i]);
            }
            panic!("unreachable");
        }))
        .collect();

    dprintln!("max_tail={}", max_tail);

    for (start, (cyc_start, ref mut path)) in paths.iter_mut() {
        let cyc_start = *cyc_start;
        let cyc_len = path.len() - cyc_start;
        let to_fill_out = max_tail - cyc_start;
        for i in path.len()..path.len() + to_fill_out {
            path.push(go(path[i - 1], dirs[i % dirs.len()]));
        }
        for div in divisors::get_divisors(cyc_len) {
            if (1..cyc_len / div)
                .all(|i| (0..div).all(|si| path[max_tail + si] == path[max_tail + i * div + si]))
            {
                dprintln!("trunc to {}", max_tail + div);
                path.truncate(max_tail + div);
                break;
            }
        }
        dprintln!(
            "start: {:?}, cyc_start={}, cyc_len={}, to_fill_out={}, path.len={}, real_cyc_len={}",
            start,
            cyc_start,
            cyc_len,
            to_fill_out,
            path.len(),
            path.len() - cyc_start
        );
    }
    let winners: HashMap<&str, Vec<usize>> = paths
        .clone()
        .into_iter()
        .map(|(start, (_, ref path))| {
            let mut wins = Vec::new();

            for i in max_tail..path.len() {
                if ends.contains(path[i]) {
                    wins.push(i - max_tail);
                }
            }

            (start, wins)
        })
        .collect();

    dprintln!("winners: {:?}", winners);

    let mut is: Vec<(&str, usize)> = starts
        .clone()
        .into_iter()
        .zip(std::iter::repeat(0))
        .collect();
    let mut mini: usize = usize::MAX;

    loop {
        let mut res = 1;
        for (v, i) in is.iter() {
            let val = winners[*v][*i];
            // dprintln!("v={}, i={}, val={}", v, i, val);
            let result = panic::catch_unwind(|| num::integer::lcm(res, val));
            if result.is_err() {
                break;
            }
            res = result.unwrap();
        }
        dprintln!("cand is {:?}", res);
        mini = mini.min(res);
        let mut ended = false;
        for (s, ref mut i) in is.iter_mut() {
            *i += 1;
            if *i == winners[s].len() {
                *i = 0;
            } else {
                ended = true;
                break;
            }
        }
        if !ended {
            break;
        }
    }

    max_tail as i64 + mini as i64
}
// 11678319315857 is the answer calculated in wolfram alpha
// by inputting lcm(*(i + max_tail for i in winners))
