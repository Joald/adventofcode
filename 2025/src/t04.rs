use crate::prelude::*;

#[allow(unused)]
pub fn solve_04(part: usize, input: String) -> i64 {
    let mut cnt = 0;
    let mut coords = Coords::parse(input);
    loop {
        let prev_cnt = cnt;
        let mut to_remove = Vec::new();
        for (x, row) in coords.iter() {
            for (y, c) in row.iter() {
                if *c == '@'
                    && neis(*x, *y, &coords, NeiDirs::Omni)
                        .iter()
                        .filter(|(_, _, c)| *c == '@')
                        .count()
                        < 4
                {
                    cnt += 1;
                    if part == 2 {
                        to_remove.push((*x, *y));
                    }
                }
            }
        }
        if part == 1 || prev_cnt == cnt {
            break;
        }
        for (x, y) in to_remove {
            if let Some(row) = coords.get_mut(&x) {
                row.insert(y, 'x');
            }
        }
    }
    cnt
}
