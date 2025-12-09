use itertools::repeat_n;

use crate::prelude::*;

fn dist(v1: &[i64], v2: &[i64]) -> f64 {
    v1.iter()
        .zip(v2)
        .map(|(x1, x2)| (x1 - x2) as f64 * (x1 - x2) as f64)
        .sum::<f64>()
        .sqrt()
}

fn find(x: usize, fun: &mut Vec<usize>, sizes: &mut Vec<usize>) -> usize {
    let val = fun[x];
    if x == val {
        return val;
    }

    let result = find(val, fun, sizes);
    fun[x] = result;
    sizes[x] = sizes[result];
    result
}

fn fun_union(x: usize, y: usize, fun: &mut Vec<usize>, sizes: &mut Vec<usize>) {
    let rx = find(x, fun, sizes);
    let ry = find(y, fun, sizes);
    fun[rx] = ry;
    fun[x] = ry;
    if rx != ry {
        let size = sizes[rx] + sizes[ry];
        sizes[rx] = size;
        sizes[ry] = size;
        sizes[x] = size;
        sizes[y] = size;
    }
}

#[allow(unused)]
pub fn solve_08(part: usize, input: String) -> i64 {
    let lines = ParsedLines::<CommaSeparated<i64>>::parse(input);
    dbg!(&lines);
    let mut distances = HashMap::<String, Vec<(usize, usize)>>::new();
    for left in 0..lines.len() {
        for right in left + 1..lines.len() {
            let dst = dist(&lines[left], &lines[right]).to_string();
            distances.entry(dst).or_default().push((left, right));
        }
    }
    let mut distances = distances
        .into_iter()
        .map(|(line, coords)| (line.parse::<f64>().unwrap(), coords))
        .collect_vec();
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits = (0..lines.len()).collect_vec();
    let mut sizes = repeat_n(1, lines.len()).collect_vec();

    for (dst, (x, y)) in distances
        .into_iter()
        .flat_map(|(dst, xys)| xys.into_iter().map(move |xy| (dst, xy)))
        .take(if part == 1 {
            if is_example() { 10 } else { 1000 }
        } else {
            usize::MAX
        })
    {
        println!("merging {x} ({:?}) with {y} ({:?})", lines[x], lines[y]);
        fun_union(x, y, &mut circuits, &mut sizes);
        if part == 2 && sizes[x] == lines.len() {
            return lines[x][0] * lines[y][0];
        }
    }

    let mut sizes_ = HashMap::<usize, i64>::new();
    for i in 0..lines.len() {
        let val = find(i, &mut circuits, &mut sizes);
        *sizes_.entry(val).or_default() += 1;
    }
    //dbg!(&sizes);
    let mut sizes = sizes_.into_values().collect_vec();
    sizes.sort();
    sizes.into_iter().rev().take(3).product()
}
