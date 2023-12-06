use crate::dbg::dprintln;
use itertools::Itertools;
use std::fmt::{Debug, Formatter, Pointer, Write};

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Repl {
    src_start: i64,
    dest_start: i64,
    len: i64,
}

impl Debug for Repl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[{:?} |-> {:?}",
            self.input_range(),
            self.output_range()
        ))
    }
}

fn is_in(x: i64, l: i64, r: i64) -> bool {
    x.clamp(l, r) == x
}

fn mkrn(first: i64, last: i64) -> Range {
    Range {
        start: first,
        len: last - first + 1,
    }
}

impl Repl {
    fn src_end(&self) -> i64 {
        self.src_start + self.len
    }
    fn src_last(&self) -> i64 {
        self.src_end() - 1
    }

    fn dest_last(&self) -> i64 {
        self.dest_start + self.len - 1
    }
    fn get(&self, x: i64) -> Option<i64> {
        if x >= self.src_start && x < self.src_start + self.len {
            // dprintln!(
            //     "{} -> {} via ({}-{}->{}-{})",
            //     x,
            //     x - self.src_start + self.dest_start,
            //     self.src_start,
            //     self.src_start + self.len - 1,
            //     self.dest_start,
            //     self.dest_start + self.len - 1
            // );
            Some(x - self.src_start + self.dest_start)
        } else {
            None
        }
    }

    fn get_range(&self, r: (Range, bool)) -> Vec<(Range, bool)> {
        let (r, changed) = r;
        if changed {
            return vec![(r, true)];
        }
        if self.input_range().intersection(r).is_none() {
            return vec![(r, changed)];
        }
        let gs = self.get(r.start);
        let gl = self.get(r.last());
        let res = if gs.is_none() && gl.is_none() {
            [
                (mkrn(r.start, self.src_start - 1), false),
                (self.output_range(), true),
                (mkrn(self.src_end(), r.last()), false),
            ]
            .into_iter()
            .filter(|&(r, _)| r.is_valid())
            .collect()
        } else if gs.is_none() {
            [
                (mkrn(r.start, self.src_start - 1), false),
                (
                    self.scoped_to(mkrn(self.src_start, r.last()))
                        .output_range(),
                    true,
                ),
            ]
            .into_iter()
            .filter(|&(r, _)| r.is_valid())
            .collect()
        } else if gl.is_none() {
            [
                (
                    self.scoped_to(mkrn(r.start, self.src_last()))
                        .output_range(),
                    true,
                ),
                (mkrn(self.src_end(), r.last()), false),
            ]
            .into_iter()
            .filter(|&(r, _)| r.is_valid())
            .collect()
        } else {
            [(self.scoped_to(r).output_range(), true)]
                .into_iter()
                .filter(|&(r, _)| r.is_valid())
                .collect()
        };
        dprintln!("{:?} -> {:?} via {:?}", r, res, self);
        res
    }

    fn input_range(&self) -> Range {
        Range {
            start: self.src_start,
            len: self.len,
        }
    }
    fn output_range(&self) -> Range {
        Range {
            start: self.dest_start,
            len: self.len,
        }
    }
    fn covers(&self, r: Range) -> bool {
        self.input_range().covers(r)
    }

    fn is_earlier_than(&self, r: Range) -> bool {
        self.src_last() < r.start
    }

    fn scoped_to(&self, r: Range) -> Repl {
        let src_start = self.src_start.max(r.start);
        let src_last = self.src_last().min(r.last());

        Repl {
            src_start,
            dest_start: src_start - self.src_start + self.dest_start,
            len: src_last - src_start + 1,
        }
    }
}

#[derive(Debug)]
struct Tbl {
    repls: Vec<Repl>,
}

impl Tbl {
    fn sorted(mut self) -> Self {
        self.repls.sort();
        self
    }
    fn get(&self, x: i64) -> i64 {
        self.repls
            .iter()
            .fold(None, |acc, repl| acc.or_else(|| repl.get(x)))
            .unwrap_or_else(|| {
                dprintln!("unmodified {}", x);
                x
            })
    }
    fn get_range(&self, rs: Ranges) -> Ranges {
        let res = Ranges {
            rs: rs
                // .clone()
                .rs
                .into_iter()
                .flat_map(|range| {
                    let mut prev = vec![(range, false)];
                    for repl in self.repls.iter().chain(vec![&Repl {
                        src_start: -10000000000000,
                        dest_start: -10000000000000,
                        len: 30000000000000,
                    }]) {
                        prev = prev
                            .into_iter()
                            .flat_map(|range| repl.get_range(range).into_iter())
                            .collect();
                    }
                    prev.into_iter().map(|(r, b)| r)
                })
                .collect(),
        };
        // dprintln!("{:?} -> {:?} via {:?}", rs.clone(), res, self);
        res
        // let mut ti = 0usize;
        // let n = self.repls.len();
        // let mut res = Ranges { rs: Vec::new() };
        // for mut range in rs.rs.into_iter() {
        //     // dprintln!("")
        //     if ti >= n {
        //         break;
        //     }
        //     while self.repls[ti].is_earlier_than(range) {
        //         ti += 1;
        //         if ti >= n {
        //             break;
        //         }
        //     }
        //     while self.repls[ti].covers(range) {
        //         let r = &self.repls[ti];
        //         let to_insert = r.scoped_to(range).output_range();
        //         dprintln!("{:?} -> {:?}", r, to_insert);
        //         res.insert(to_insert, false);
        //         ti += 1;
        //         if ti >= n {
        //             break;
        //         }
        //     }
        // }
        // res.sorted()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct Range {
    start: i64,
    len: i64,
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("[{}-{}]", self.start, self.last()))
    }
}

impl Range {
    fn first_out(&self) -> i64 {
        self.start + self.len
    }
    fn last(&self) -> i64 {
        self.first_out() - 1
    }
    fn is_valid(&self) -> bool {
        self.len >= 0
    }
    fn intersection(&self, other: Range) -> Option<Range> {
        let start = self.start.max(other.start);
        let last = self.last().min(other.last());
        if last < start {
            None
        } else {
            Some(Range {
                start,
                len: last - start + 1,
            })
        }
    }

    fn covers(&self, other: Range) -> bool {
        is_in(other.start, self.start, self.last())
            || is_in(other.last(), self.start, self.last())
            || is_in(self.start, other.start, other.last())
            || is_in(self.last(), other.start, other.last())
    }
    fn is_adjacent_to(&self, other: Range) -> bool {
        self.start - 1 == other.last() || self.last() + 1 == other.start
    }
    fn prefix_of(&self, other: Range) -> Option<Range> {
        if self.start < other.start {
            Some(Range {
                start: self.start,
                len: other.start - self.start + 1,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Ranges {
    rs: Vec<Range>,
}

impl Ranges {
    fn from_pair_iter(it: impl Iterator<Item = i64>) -> Self {
        let (start, len) = it.collect_tuple().unwrap();
        Ranges {
            rs: vec![Range { start, len }],
        }
    }
}

fn parse_5(lines: Vec<String>) -> (Vec<i64>, Vec<Tbl>) {
    let mut init_seeds: Vec<_> = lines
        .iter()
        .take(1)
        .next()
        .unwrap()
        .split(" ")
        .skip(1)
        .map(str::parse::<i64>)
        .map(Result::unwrap)
        .collect();

    let tbls = (&lines)
        .split(|s| s.is_empty())
        .skip(1)
        .map(|slice| {
            Tbl {
                repls: slice
                    .iter()
                    .skip(1)
                    .map(|line| {
                        let (dest_start, src_start, len) = line
                            .split(" ")
                            .map(str::parse::<i64>)
                            .map(Result::unwrap)
                            .collect_tuple()
                            .unwrap();
                        Repl {
                            dest_start,
                            src_start,
                            len,
                        }
                    })
                    .collect(),
            }
            .sorted()
        })
        .collect();
    (init_seeds, tbls)
}

pub fn solve_5(part: usize, lines: Vec<String>) -> i64 {
    let (inits, tbls) = parse_5(lines);
    if part == 1 {
        inits
            .iter()
            .map(|seed| {
                dprintln!("\n==== seed {} ====", seed);
                tbls.iter().fold(*seed, |acc, tbl| tbl.get(acc))
            })
            .min()
            .unwrap()
    } else {
        inits
            .into_iter()
            .chunks(2)
            .into_iter()
            .map(|chunk| {
                let ranges = Ranges::from_pair_iter(chunk);
                dprintln!("\n==== seed {:?} ====", ranges);
                tbls.iter().fold(ranges, |acc, tbl| {
                    let res = tbl.get_range(acc);
                    dprintln!("---> new ranges {:?}", res);
                    res
                })
            })
            .map(|r| r.rs.into_iter().map(|r| r.start).min().unwrap())
            .min()
            .unwrap()
    }
}
