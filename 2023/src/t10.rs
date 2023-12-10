use crate::dbg::dprintln;
use colored::*;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn all_neis(lines: &[Vec<char>], x: usize, y: usize) -> (Vec<(usize, usize)>, bool) {
    let mut res = vec![];
    let mut bound = false;
    if x > 0 {
        res.push((x - 1, y));
    } else {
        bound = true;
    }
    if y > 0 {
        res.push((x, y - 1));
    } else {
        bound = true;
    }
    if x < lines.len() - 1 {
        res.push((x + 1, y));
    } else {
        bound = true;
    }
    if y < lines[x].len() - 1 {
        res.push((x, y + 1));
    } else {
        bound = true;
    }
    (res, bound)
}

fn get_neis(lines: &[Vec<char>], x: usize, y: usize) -> Vec<(usize, usize)> {
    match lines[x][y] {
        '|' => vec![(x - 1, y), (x + 1, y)],
        '-' => vec![(x, y - 1), (x, y + 1)],
        'F' => vec![(x, y + 1), (x + 1, y)],
        'L' => vec![(x, y + 1), (x - 1, y)],
        '7' => vec![(x, y - 1), (x + 1, y)],
        'J' => vec![(x, y - 1), (x - 1, y)],
        'S' => {
            let mut res = Vec::new();
            if x > 0 && "|F7".contains(lines[x - 1][y]) {
                res.push((x - 1, y));
            }
            if y > 0 && "-FL".contains(lines[x][y - 1]) {
                res.push((x, y - 1));
            }
            if x < lines.len() - 1 && "|JL".contains(lines[x + 1][y]) {
                res.push((x + 1, y));
            }
            if y < lines[x].len() - 1 && "-J7".contains(lines[x][y + 1]) {
                res.push((x, y + 1))
            }
            res
        }
        _ => panic!("Invalid char {} at ({}, {})", lines[x][y], x, y),
    }
}

fn print_board(lines: &[Vec<char>]) {
    for line in lines {
        println!("{}", line.iter().join(""));
    }
}

fn print_loop(lines: &[Vec<char>], lup: &HashSet<(usize, usize)>) {
    for x in 0..lines.len() {
        for y in 0..lines[x].len() {
            print!(
                "{}",
                if lup.contains(&(x, y)) {
                    format!("{}", lines[x][y]).red()
                } else {
                    format!("{}", lines[x][y]).normal()
                }
            );
        }
        print!("\n");
    }
}

#[allow(unused)]
pub fn solve_10(part: usize, lines: Vec<String>) -> i64 {
    let lines: Vec<Vec<char>> = lines
        .iter()
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().collect())
        .collect();
    print_board(&lines);
    let lines = if part == 1 {
        lines
    } else {
        let mut new_lns = Vec::from([vec!['#'; 2 * lines[0].len() + 1]]);
        for x in 0..lines.len() {
            new_lns.push(Vec::new());
            let row = new_lns.last_mut().unwrap();
            row.push('#');
            for y in 0..lines[x].len() {
                row.push(lines[x][y]);
                if y < lines[x].len() - 1
                    && "SL-F".contains(lines[x][y])
                    && "S7-J".contains(lines[x][y + 1])
                {
                    row.push('-');
                } else {
                    row.push('#');
                }
            }
            new_lns.push(Vec::new());
            let extra_row = new_lns.last_mut().unwrap();
            extra_row.push('#');
            for y in 0..lines[x].len() {
                if x < lines.len() - 1
                    && "SF|7".contains(lines[x][y])
                    && "SJ|L".contains(lines[x + 1][y])
                {
                    extra_row.push('|');
                } else {
                    extra_row.push('#');
                }
                extra_row.push('#');
            }
        }
        new_lns
    };
    print_board(&lines);
    let (sx, sy) = {
        let mut sx = 0;
        let mut sy = 0;
        for i in 0..lines.len() {
            for j in 0..lines[i].len() {
                if lines[i][j] == 'S' {
                    sx = i;
                    sy = j;
                    break;
                }
            }
        }
        (sx, sy)
    };
    let mut vis = HashSet::from([(sx, sy)]);
    let mut q: VecDeque<((usize, usize), i64)> = VecDeque::from([((sx, sy), 0)]);
    let mut maxdist = 0;
    while let Some(((x, y), dist)) = q.pop_front() {
        maxdist = maxdist.max(dist);
        for (nx, ny) in get_neis(&lines, x, y) {
            if !vis.contains(&(nx, ny)) {
                vis.insert((nx, ny));
                q.push_back(((nx, ny), dist + 1));
            }
        }
    }
    if part == 1 {
        maxdist
    } else {
        let lup = vis;
        print_loop(&lines, &lup);
        let fresh: Vec<_> = lines.iter().map(|l| vec![i32::MAX; l.len()]).collect();
        let mut incount = 0;
        for x in 0..lines.len() {
            for y in 0..lines[x].len() {
                if lup.contains(&(x, y))
                    || x == 0
                    || y == 0
                    || x == lines.len() - 1
                    || y == lines[x].len() - 1
                    || lines[x][y] == '#'
                    || x % 2 == 0
                    || y % 2 == 0
                {
                    continue;
                }
                dprintln!("considering {:?}", (x, y));
                let mut vis = HashSet::new();
                vis.insert((x, y));
                let mut q = VecDeque::from([(x, y)]);
                while let Some((x, y)) = q.pop_front() {
                    dprintln!("  visiting {:?} with '{}'", (x, y), lines[x][y]);
                    let (neis, bound) = all_neis(&lines, x, y);
                    dprintln!("  neis are: {:?}", neis);
                    if bound {
                        dprintln!("  bound reached, exiting!");

                        incount -= 1;
                        dprintln!("  it was outside!");
                        break;
                    }
                    for (nx, ny) in neis {
                        if lup.contains(&(nx, ny)) || vis.contains(&(nx, ny)) {
                            continue;
                        }
                        q.push_back((nx, ny));
                        vis.insert((nx, ny));
                    }
                }
                incount += 1;
            }
        }
        incount
    }
}
