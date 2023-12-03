use crate::dbg::dprintln;

// TASK 1

const NUMS: [&'static str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_calibration(s: &str, first: bool, part2: bool) -> i64 {
    let b = s.as_bytes();
    let is_digit = |c: char| c.is_digit(10);
    if !part2 {
        (b[if first {
            s.find(is_digit).unwrap()
        } else {
            s.rfind(is_digit).unwrap()
        }] - '0' as u8)
            .into()
    } else {
        let worst_index = if first { usize::MAX } else { usize::MIN };
        let index = NUMS
            .iter()
            .filter_map(|num| Some((num, if first { s.find(num)? } else { s.rfind(num)? })));
        let (numstr, index) = if first {
            index.min_by_key(|(_, i)| *i)
        } else {
            index.max_by_key(|(_, i)| *i)
        }
        .unwrap_or((&"", worst_index));
        let numindex = if first {
            s.find(is_digit).unwrap_or(worst_index)
        } else {
            s.rfind(is_digit).unwrap_or(worst_index)
        };
        if (numindex < index) == first {
            find_calibration(s, first, false)
        } else {
            NUMS.iter().position(|elem| elem == numstr).unwrap() as i64
        }
    }
}

fn extract(s: String) -> (i64, i64) {
    (
        find_calibration(&s, true, true),
        find_calibration(&s, false, true),
    )
}

pub fn solve_1(_part: usize, lines: Vec<String>) -> i64 {
    lines
        .into_iter()
        .map(extract)
        .map(|(t, u)| 10 * t + u)
        .sum()
}

// TASK 2

#[derive(Default)]
struct Reveal {
    rs: i32,
    gs: i32,
    bs: i32,
}

impl Reveal {
    fn power_level(&self) -> i64 {
        self.rs as i64 * self.gs as i64 * self.bs as i64
    }
}

struct Game {
    num: i32,
    rs: Vec<Reveal>,
}

impl Game {
    fn is_valid(&self, r: Reveal) -> bool {
        !self
            .rs
            .iter()
            .any(|ri| ri.rs > r.rs || ri.gs > r.gs || ri.bs > r.bs)
    }

    fn fewest(&self) -> Reveal {
        Reveal {
            rs: self.rs.iter().map(|r| r.rs).max().unwrap_or(0),
            gs: self.rs.iter().map(|r| r.gs).max().unwrap_or(0),
            bs: self.rs.iter().map(|r| r.bs).max().unwrap_or(0),
        }
    }
}

fn game_num(g: &Game) -> i64 {
    g.num as i64
}

fn parse_2(lines: Vec<String>) -> Vec<Game> {
    lines
        .into_iter()
        .map(|line| {
            let (pref, rs) = line.split_once(": ").unwrap();
            let num: i32 = pref["GAME ".len()..].parse().unwrap();
            let rs = rs
                .split("; ")
                .map(|r| {
                    let mut rev = Reveal::default();
                    for c in r.split(", ") {
                        let (num, col) = c.trim().split_once(" ").unwrap();
                        let num: i32 = num.parse().unwrap();
                        match col {
                            "red" => rev.rs = num,
                            "blue" => rev.bs = num,
                            "green" => rev.gs = num,
                            _ => panic!("invalid color {}", col),
                        }
                    }
                    rev
                })
                .collect();
            Game { num, rs }
        })
        .collect()
}

pub fn solve_2(part: usize, lines: Vec<String>) -> i64 {
    let games = parse_2(lines);
    if part == 1 {
        games
            .iter()
            .filter(|game| {
                game.is_valid(Reveal {
                    rs: 12,
                    gs: 13,
                    bs: 14,
                })
            })
            .map(game_num)
            .sum()
    } else {
        games.iter().map(|game| game.fewest().power_level()).sum()
    }
}

// TASK 3

#[derive(Debug)]
struct Symbol {
    x: usize,
    y: usize,
    gear: bool,
}

#[derive(Debug)]
struct Number {
    x: usize,
    y_fst: usize,
    y_lst: usize,
    val: i64,
}

impl Number {
    fn is_adjacent_to(&self, sym: &Symbol) -> bool {
        let res = self.x.abs_diff(sym.x) <= 1
            && (self.y_fst..self.y_lst + 1).any(|y| y.abs_diff(sym.y) <= 1);
        dprintln!(
            "{:?} is {}adjacent to {:?}",
            self,
            if res { "" } else { "not " },
            sym
        );
        res
    }
}

fn parse_3(lines: Vec<String>) -> (Vec<Symbol>, Vec<Number>) {
    let mut syms = Vec::new();
    let mut nums = Vec::new();
    for (x, line) in lines.iter().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            if c.is_digit(10) {
                let val = c as i64 - '0' as i64;
                if nums.is_empty() || {
                    let prev: &Number = nums.last().unwrap();
                    prev.x != x || prev.y_lst != y - 1
                } {
                    nums.push(Number {
                        x,
                        y_fst: y,
                        y_lst: y,
                        val,
                    })
                } else {
                    let prev = nums.last_mut().unwrap();
                    prev.val = prev.val * 10 + val;
                    prev.y_lst += 1;
                }
            } else {
                syms.push(Symbol {
                    x,
                    y,
                    gear: c == '*',
                })
            }
        }
    }
    (syms, nums)
}

pub fn solve_3(part: usize, lines: Vec<String>) -> i64 {
    let (syms, nums) = parse_3(lines);
    dprintln!("syms={:?}, nums={:?}", syms, nums);

    if part == 1 {
        nums.iter()
            .filter(|num| syms.iter().any(|sym| num.is_adjacent_to(sym)))
            .map(|num| num.val)
            .sum()
    } else {
        syms.iter()
            .filter_map(|sym| {
                if !sym.gear {
                    return None;
                }
                let adj_nums: Vec<_> = nums.iter().filter(|num| num.is_adjacent_to(sym)).collect();
                if adj_nums.len() != 2 {
                    return None;
                }
                let res: i64 = adj_nums.iter().map(|num| num.val).product();
                Some(res)
            })
            .sum()
    }
}

// TASK 4
