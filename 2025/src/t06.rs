use crate::prelude::*;

fn was_looped(
    mut sx: i64,
    mut sy: i64,
    coords: &CoordsResult,
    vis: &mut HashMap<(i64, i64), Vec<(i64, i64)>>,
) -> bool {
    let h = coords.len() as i64;
    let w = coords[&0].len() as i64;
    let (mut dirx, mut diry) = (-1, 0);
    macro_rules! next_dir {
        () => {
            dirx *= -1;
            (dirx, diry) = (diry, dirx);
        };
    }

    loop {
        let (nx, ny) = (sx + dirx, sy + diry);
        if !(0..h).contains(&nx) || !(0..w).contains(&ny) {
            return false;
        }
        if vis
            .get(&(nx, ny))
            .is_some_and(|v| v.contains(&(dirx, diry)))
        {
            return true;
        }
        if !coords.contains_key(&nx) || !coords[&nx].contains_key(&ny) {
            dbg!(sx, sy, dirx, diry, nx, ny, h, w);
            println!("coord={:?}\nvis={:?}",&coords, &vis);
        }
        let c = coords[&nx][&ny];
        if c == '#' {
            next_dir!();
        } else {
            (sx, sy) = (nx, ny);
            vis.entry((sx, sy)).or_default().push((dirx, diry));
        }
    }
}

#[allow(unused)]
pub fn solve_06(part: usize, input: String) -> i64 {
    let mut coords = Coords::parse(input);
    let h = coords.len() as i64;
    let w = coords[&0].len() as i64;

    let mut sx = -1;
    let mut sy = -1;
    'outer: for (x, row) in &coords {
        for (y, c) in row {
            if *c == '^' {
                sx = *x;
                sy = *y;
                break 'outer;
            }
        }
    }

    let mut vis = HashMap::from([((sx, sy), vec![(-1, 0)])]);
    if part == 1 {
        was_looped(sx, sy, &coords, &mut vis);
        vis.len() as i64
    } else {
        let fresh_vis = vis.clone();
        (0..h)
            .cartesian_product(0..w)
            .map(|(x, y)| {
                let spot = coords.get_mut(&x).unwrap().get_mut(&y).unwrap();
                if *spot == '.' {
                    *spot = '#';
                    let res = was_looped(sx, sy, &coords, &mut vis);
                    *coords.get_mut(&x).unwrap().get_mut(&y).unwrap() = '.';
                    vis = fresh_vis.clone();
                    res as i64
                } else {
                    0
                }
            })
            .sum()
    }
}
