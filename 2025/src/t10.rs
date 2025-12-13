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
            println!("{}, {}", buttons.len(), joltages.len());
            return 0;
            println!(
                "Considering line {}: {:?} with {} buttons {buttons:?}",
                i + 1,
                joltages,
                buttons.len()
            );
            if let Some(cached_result) = get_cached_line_result(i + 1) {
                return cached_result;
            }
            if var_or("GJ", "0") == 1 {
                let n = joltages.len();
                #[allow(non_snake_case)]
                let mut A: Vec<Vec<f64>> = (0..n).map(|i| (0..buttons.len()).map(|bi| if buttons[bi].contains(&i) {1.} else {0.}).collect_vec()).collect_vec();
                for i in 0..n {
                    A[i].push(joltages[i] as f64);
                }
                for schodek in 0..n {
                    // invariant: [0..schodek) are already triangular
                    let to_swap = (schodek..n).find(|j| A[schodek][*j] != 0.).unwrap();
                    A.swap(schodek, to_swap);
                    let lead_val = A[schodek][schodek];
                    for val in A[schodek].iter_mut() {
                        *val /= lead_val;
                    }
                    for other_i in schodek+1..n {
                        let lead_val = A[other_i][schodek];
                        A[other_i][schodek] = 0.;
                        for  other_j in schodek+1..A[other_i].len() {
                            A[other_i][other_j] -= A[schodek][other_j] * lead_val;
                        }
                    }
                }
                for schodek in (0..n).rev() {
                    // invariant: (schodek; n) is diagonalized
                    // 

                    
                }
                return A.into_iter().map(|row| *row.last().unwrap()).sum::<f64>() as i64
            }

            if var_or("PARTITIONS", "0") == 1 {
            let mut possibilities: Vec<Vec<usize>> = vec![vec![0; buttons.len()]];
            let mut done_once = false;

            let mut considered_joltages = Vec::new();
            for (joltages_i, current_joltage) in joltages.iter().enumerate().sorted_by_key(|(_, j)| buttons.iter().filter(|btn| btn.contains(j)).count() * 1000 + *j) {
                // invariant: possibilities contains all non-overflown options that satisfy [0;i)
                // i.e. for each possibilities[i], if you press button x possibilities[i][x] times,
                // all joltages 0 <= j < i will be on target
                for cand in possibilities.iter() {
                    let mut rs = vec![0; joltages.len()];
                    for (btn, times) in cand.iter().enumerate() {
                        for val in buttons[btn].iter() {
                            rs[*val] += times;
                        }
                    }
                    for jj in considered_joltages.iter() {
                        assert!(rs[*jj] == joltages[*jj]);
                    }
                }
                considered_joltages.push(joltages_i);

                let mut affecting_btns = Vec::new();
                for (btn_i, btn) in buttons.iter().enumerate() {
                    if btn.contains(&joltages_i)
                    {
                        affecting_btns.push(btn_i);
                    }
                }

                println!(
                    "Considering joltage #{joltages_i} ({current_joltage}) with {} possibilities and affecting_btns={affecting_btns:?}",
                    possibilities.len()
                );

                possibilities = possibilities
                    .into_iter()
                    //.skip(10000)
                    //.take(10000)
                    .progress()
                    .flat_map(|mut cand| {
                        let mut result = vec![0; joltages.len()];
                        for (btn, times) in cand.iter().enumerate() {
                            for joltage in buttons[btn].iter() {
                                result[*joltage] += times;
                            }
                        }
                        //println!("Candidate {cand:?} with results {result:?}");
                        let mut new_cands = Vec::new();
                        let to_fill = current_joltage - result[joltages_i];
                        let mut btn_selection = vec![0; affecting_btns.len()];
                        btn_selection[0] = to_fill;
                        cand[affecting_btns[0]] += to_fill;
                        for j_i in buttons[affecting_btns[0]].iter() {
                            result[*j_i] += to_fill;
                        }

                        // |btns|-compositions of to_fill:
                        // k-compositions of n can be expressed recursively as
                        // *[[*i, 1] for i in [k-1-compositions of n-1]],
                        // *[[*i, 2] for i in [k-1-compositions of n-2]], ...,
                        // *[[*i, n-1] for i in [k-1-compositions of 1]],
                        // [0, 0, ..., 0, n]
                        // so the last one will always have a zero-prefix
                        // at which point we go to the next batch of compositions
                        // one level higher
                        // and we restart the "current level" from scratch
                        // (but with one fewer "piece of fuel", which is the piece we moved up)
                        // (because total sum must be constant)
                        loop {
                            //println!(
                            //    "btn_selection={btn_selection:?}, cand={cand:?}, result={result:?}"
                            //);
                            if result.iter().enumerate().all(|(i, x)| *x <= joltages[i]) {
                                //println!("push!");
                                new_cands.push(cand.clone());
                            }
                            if *btn_selection.last().unwrap() == to_fill {
                                break;
                            }

                            let i = btn_selection
                                .iter()
                                .enumerate()
                                .find(|x| *x.1 > 0)
                                .unwrap()
                                .0
                                + 1;

                            //println!("{i}");

                            btn_selection[i] += 1;
                            cand[affecting_btns[i]] += 1;
                            for j_i in buttons[affecting_btns[i]].iter() {
                                result[*j_i] += 1;
                            }

                            for j_i in buttons[affecting_btns[0]].iter() {
                                if i - 1 == 0 {
                                    // just decrement
                                    result[*j_i] -= 1;
                                } else {
                                    result[*j_i] += btn_selection[i - 1] - 1;
                                }
                            }
                            if i - 1 == 0 {
                                cand[affecting_btns[0]] -= 1;
                            } else {
                                cand[affecting_btns[0]] += btn_selection[i - 1] - 1;
                                cand[affecting_btns[i - 1]] -=1;
                            }
                            btn_selection[0] = btn_selection[i - 1] - 1;

                            if i > 1 {
                                // we are not moving from [0]
                                // so we need to begin the "current level"
                                // by moving things back to [0]
                                for j_i in buttons[affecting_btns[i - 1]].iter() {
                                    result[*j_i] -= btn_selection[i - 1]; // -1;
                                }
                                //println!("i-1={}, affecting_btns[i-1]={}, cand[affecting_btns[i-1]]={}", i-1,affecting_btns[i-1],cand[affecting_btns[i-1]]);
                                cand[affecting_btns[i - 1]] -= btn_selection[i - 1] - 1;
                                btn_selection[i - 1] = 0;
                            }
                        }
                        new_cands
                    })
                    .collect_vec();
            }
            let res =  possibilities
                .into_iter()
                .map(|cand| cand.iter().sum::<usize>())
                .min()
                .unwrap() as i64;
            println!("result -> {res}");
            //save_line_result_to_cache(i+1, res);
            return res;
            }

            let n = buttons.len();
            let mut result = vec![0; diagram.len()];

            let mut possibilities: Vec<Vec<usize>> = vec![Vec::new()];
            'outer: loop {
                let mut buf = Vec::new();
                for cand in std::mem::take(&mut possibilities).into_iter() {
                    let start = cand.last().copied().unwrap_or(0);
                    for i in start..n {
                        let mut new_cand = cand.clone();
                        new_cand.push(i);
                        buf.push(new_cand);
                    }
                }
                std::mem::swap(&mut possibilities, &mut buf);
                println!(
                    "Considering {} possibilities of len {}",
                    possibilities.len(),
                    possibilities[0].len()
                );
                let mut found = false;
                possibilities = possibilities
                    .into_iter()
                    .filter_map(|cand| {
                        if found {
                            return None;
                        }
                        for i in cand.iter() {
                            for x in buttons[*i].iter() {
                                if part == 1 {
                                    result[*x] = 1 - result[*x];
                                } else {
                                    result[*x] += 1;
                                }
                            }
                        }
                        if (part == 1 && result == diagram) || (part == 2 && result == joltages) {
                            found = true;
                        }
                        let new_cand = if part == 2
                            && result.iter().enumerate().any(|(i, x)| joltages[i] < *x)
                        {
                            None
                        } else {
                            Some(cand)
                        };
                        result.fill(0);
                        new_cand
                    })
                    .collect_vec();
                if found {
                    break;
                }
            }

            let result = possibilities[0].len() as i64;
            println!("Line {} done -> {result}", i + 1);
            save_line_result_to_cache(i+1, result);
            result
        })
        .sum()
}
