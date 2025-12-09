#[allow(unused)]
use crate::prelude::*;
#[allow(unused)]
pub fn solve_09(part: usize, input: String) -> i64 {
    let vals = ParsedLines::<CommaSeparated<i64>>::parse(input);
    //let (minx, miny, maxx, maxy) = vals.iter().fold(
    //    (i64::MAX, i64::MAX, i64::MIN, i64::MIN),
    //    |(acc_minx, acc_miny, acc_maxx, acc_maxy), v| {
    //        let (x, y) = (v[0], v[1]);
    //        (
    //            acc_minx.min(x),
    //            acc_miny.min(y),
    //            acc_maxx.max(x),
    //            acc_maxy.max(y),
    //        )
    //    },
    //);
    //let mut greens = HashSet::<(i64, i64)>::new();
    //for i in 0..vals.len() {
    //    let lx = vals[i][0];
    //    let rx = vals[(i + 1) % vals.len()][0];
    //    let ly = vals[i][1];
    //    let ry = vals[(i + 1) % vals.len()][1];
    //
    //    if lx == rx {
    //        for j in ly.min(ry)..ly.max(ry) + 1 {
    //            greens.insert((lx, j));
    //        }
    //    } else {
    //        // ly == ry
    //        for j in lx.min(rx)..lx.max(rx) + 1 {
    //            greens.insert((j, ly));
    //        }
    //    }
    //}
    //let init = if is_example() {
    //    (10, 2)
    //} else {
    //    (98161, 50090)
    //};
    //let mut interior = HashSet::from([init]);
    //let mut q = Vec::new();
    //q.push(init);
    //while let Some((x, y)) = q.pop() {
    //    //dbg!(x, y, q.len());
    //    // invariant: (x, y) is inserted into interior
    //    for (xdelta, ydelta) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
    //        let (xi, yi) = (x + xdelta, y + ydelta);
    //        if xi < minx || xi > maxx || yi < miny || yi > maxy {
    //            panic!("Bad init!");
    //        }
    //        if !greens.contains(&(xi, yi)) && !interior.contains(&(xi, yi)) {
    //            interior.insert((xi, yi));
    //            q.push((xi, yi));
    //        }
    //    }
    //}
    //
    //println!(
    //    "{}",
    //    (98150..98170)
    //        .map(|x| (50050..50100)
    //            .map(|y| if greens.contains(&(y, x)) || interior.contains(&(y, x)){ '#' } else { '.' })
    //            .join(""))
    //        .join("\n")
    //);
    //
    let segments = vals
        .iter()
        .zip(vals.iter().skip(1).chain([&vals[0]]))
        .map(|(v1, v2)| {
            let (sx, sy, tx, ty) = (v1[0], v1[1], v2[0], v2[1]);
            
            let (sx, sy, tx, ty) = if sx > tx || sy > ty {
                (tx, ty, sx, sy)
            } else {
                (sx, sy, tx, ty)
            };
            assert!(sx <= tx && sy <= ty);        
            (sx, sy, tx, ty)
        })
        .collect_vec();

    let mut result = 0;
    for i in 0..vals.len() {
        for j in i + 1..vals.len() {
            let lx = vals[i][0];
            let rx = vals[j][0];
            let ly = vals[i][1];
            let ry = vals[j][1];
            let val = ((vals[i][0] - vals[j][0]).abs() + 1) * ((vals[i][1] - vals[j][1]).abs() + 1);
            //if val <= 450734120 {
            //    continue;
            //}
            if part == 1
                || [
                    (lx, ly, lx, ry),
                    (lx, ly, rx, ly),
                    (lx, ry, rx, ry),
                    (rx, ly, rx, ry),
                ]
                .iter()
                .all(|(sx, sy, tx, ty)| {
                    // w.l.o.g: sx <= tx && sy <= ty
                    let (sx, sy, tx, ty) = if sx > tx || sy > ty {
                        (tx, ty, sx, sy)
                    } else {
                        (sx, sy, tx, ty)
                    };
                    assert!(sx <= tx && sy <= ty);
                    segments.iter().all(|(six, siy, tix, tiy)| {
                        // w.l.o.g: six <= tix && siy <= tiy (done at creation)
                        //let (six, siy, tix, tiy) = if six > tix || siy > tiy {
                        //    (tix, tiy, six, siy)
                        //} else {
                        //    (six, siy, tix, tiy)
                        //};
                        assert!(six <= tix && siy <= tiy);
                        // w.l.o.g: flip axes so that sx == tx
                        let (sx, sy, tx, ty, six, siy, tix, tiy, flipped) = if sy == ty {
                            (sy, sx, ty, tx, siy, six, tiy, tix, true)
                        } else {
                            (sx, sy, tx, ty, six, siy, tix, tiy, false)
                        };
                        assert!(sx == tx);
                        // case 1: not intersecting at all
                        six > tx || sx > tix || siy > ty || sy > tiy || 
                        // case 2: NEGATIVE: intersecting and -|- (T-intersection in the middle)
                        !(six < sx && tix > sx && sy < siy && siy < ty) && 
                        // case 3: NEGATIVE we are overlapping the line from both ends
                        !(six == tix && sy < siy && ty > tiy) && (
                        // case 4: we are fully on a line
                        six == tix && sy >= siy && ty <= tiy 
                        // case 5: FINAL we have a dangling end we need to catch
                        || {
                            let (x, y) = if six == tix {
                                // case 5.1: we are parallel and sticking out on one end
                                if sy < siy {
                                    (sx, sy)
                                } else {
                                    assert!(ty > tiy);
                                    (tx, ty)}
                            } else {
                                // case 5.2: we are perpendicular and sticking out on one side
                                // Either the edge is at segment's x-endpoint, or segment touches edge's y-endpoint
                                if siy == sy {
                                    // Segment touches at edge's lower y - dangling is upper
                                    (tx, ty)
                                } else if siy == ty {
                                    // Segment touches at edge's upper y - dangling is lower
                                    (sx, sy)
                                } else if sx == six {
                                    // Edge at segment's left x
                                    (tx, ty)
                                } else {
                                    // Edge at segment's right x
                                    assert!(tix == tx);
                                    (sx, sy)
                                }
                            };
                            let (x, y) = if flipped {
                                (y, x)
                            } else {
                                (x, y)
                            };

                            // w.l.o.g: shooting a beam towards positive x
                            let mut fuck = false;
                            let mut hits = 0;
                            for (sx,sy,tx,ty) in segments.iter() {
                                if sy == y && ty == y && sx > x {
                                    hits = 0;
                                    fuck = true;
                                    break;
                                } else if sx > x && sy < y && y < ty {
                                    hits += 1;
                                } 
                            }
                            if fuck {
                                // try negative x?
                                fuck = false;
                                for (sx, sy, tx, ty) in segments.iter() {
                                    if sy == y && ty == y  && tx < x {
                                        hits = 0;
                                        fuck = true;
                                        break;
                                    } else if tx < x && sy < y && y < ty {
                                        hits += 1;
                                    }
                                }
                                if fuck {
                                    // grrr. positive y?
                                    fuck = false;
                                    for (sx, sy, tx, ty) in segments.iter() {
                                        if sx == x && tx == x && sy > y {
                                            hits = 0;
                                            fuck = true;
                                            break;
                                        } else if sy > y && sx < x && x < tx {
                                            hits += 1;
                                        }
                                    }
                                    if fuck {
                                        // negative y?
                                        fuck = false;
                                        for (sx, sy, tx, ty) in segments.iter() {
                                            if sx == x && tx == x && ty < y {
                                                hits = 0;
                                                fuck = true;
                                                break;
                                            } else if ty < y && sx < x && x < tx {
                                                hits += 1;
                                            }
                                        }
                                        if fuck {
                                            panic!("fuck.");
                                        }
                                    }
                                }
                            }
                            if hits % 2 == 0 {

                                //println!("rejecting potentially-good value {val} for lx={lx}, ly={ly}, rx={rx}, ry={ry}");
                            }
                            hits% 2 == 1

                        }
                        )
                    })
                })
            {
                result = result.max(val);
            }
        }
    }
    result
}
// first answer: too low 450734120
