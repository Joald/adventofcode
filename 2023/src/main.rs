use anyhow::Result;
const DEBUG: bool = false;

macro_rules! dprintln {
    ($($arg:expr),+) => {
        if DEBUG {
            eprintln!($($arg),*);
        }
    };
}

fn read_input(task_num: usize) -> Result<Vec<String>> {
    Ok(
        std::fs::read_to_string(format!("tasks/{:0>2}/input.txt", task_num))?
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect(),
    )
}

fn ex_input_1_1() -> &'static str {
    "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
}

fn ex_input_1_2() -> &'static str {
    "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
}

fn ex_input_2() -> &'static str {
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
}

fn ex_input_3() -> &'static str {
    "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
}

fn ex_input(task_num: usize, part: usize) -> Vec<String> {
    let inps = [
        [ex_input_1_1(), ex_input_1_2()],
        [ex_input_2(), ex_input_2()],
        [ex_input_3(), ex_input_3()],
    ];
    inps[task_num - 1][part - 1]
        .split("\n")
        .map(|s| s.to_owned())
        .collect()
}

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

fn solve_1(_part: usize, lines: Vec<String>) -> i64 {
    lines
        .into_iter()
        .map(extract)
        .map(|(t, u)| 10 * t + u)
        .sum()
}

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

fn solve_2(part: usize, lines: Vec<String>) -> i64 {
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
        dprintln!("{:?} is {}adjacent to {:?}", self, if res {""} else {"not "}, sym);
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
                syms.push(Symbol { x, y, gear: c == '*' })
            }
        }
    }
    (syms, nums)
}

fn solve_3(part: usize, lines: Vec<String>) -> i64 {
    let (syms, nums) = parse_3(lines);
    dprintln!("syms={:?}, nums={:?}", syms, nums);

    if part == 1 {
        nums.iter()
            .filter(|num| syms.iter().any(|sym| num.is_adjacent_to(sym)))
            .map(|num| num.val)
            .sum()
    } else {
        syms.iter().filter_map(|sym| {
            if !sym.gear {
                return None;
            }
            let adj_nums: Vec<_> = nums.iter().filter(|num| num.is_adjacent_to(sym)).collect();
            if adj_nums.len() != 2 {
                return None;
            }
            let res: i64 = adj_nums.iter().map(|num| num.val).product();
            Some(res)
        }).sum()
    }
}

fn solve(task_num: usize, part: usize, lines: Vec<String>) -> i64 {
    let tasks = [solve_1, solve_2, solve_3];
    tasks[task_num - 1](part, lines)
}

fn main() -> Result<()> {
    const TASK_NUM: usize = 3;
    const PART: usize = 2;
    const EXAMPLE: bool = true;
    println!(
        "{}",
        solve(
            TASK_NUM,
            PART,
            if EXAMPLE {
                ex_input(TASK_NUM, PART)
            } else {
                read_input(TASK_NUM)?
            }
        )
    );
    Ok(())
}
