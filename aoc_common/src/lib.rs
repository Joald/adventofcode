use core::panic;
use itertools::Itertools;
use serde_json::Value;
use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

pub fn var_or(var: &str, or: &str) -> usize {
    std::env::var(var)
        .unwrap_or(or.to_string())
        .parse::<usize>()
        .unwrap()
}

pub fn is_example() -> bool {
    std::env::var("EXAMPLE").is_ok_and(|val| val == "1")
}

pub trait InputParser {
    type Res;
    fn parse(input: String) -> Self::Res;
}

pub struct Lines {}
impl InputParser for Lines {
    type Res = Vec<String>;
    fn parse(input: String) -> Vec<String> {
        input.split("\n").map(str::to_owned).collect()
    }
}

pub type CoordsResult = HashMap<i64, HashMap<i64, char>>;
pub struct Coords {}
impl InputParser for Coords {
    type Res = CoordsResult;
    fn parse(input: String) -> Self::Res {
        Lines::parse(input)
            .iter()
            .enumerate()
            .map(|(x, l)| {
                let row = l.chars().enumerate().map(|(y, c)| (y as i64, c)).collect();
                (x as i64, row)
            })
            .collect()
    }
}

pub struct Blocks {}
impl InputParser for Blocks {
    type Res = Vec<Vec<String>>;
    fn parse(input: String) -> Self::Res {
        input
            .split("\n\n")
            .map(|block| block.split("\n").map_into().collect_vec())
            .collect_vec()
    }
}

pub struct TwoBlocks<B1: InputParser, B2: InputParser> {
    marker1: PhantomData<B1>,
    marker2: PhantomData<B2>,
}
impl<B1: InputParser, B2: InputParser> InputParser for TwoBlocks<B1, B2> {
    type Res = (B1::Res, B2::Res);
    fn parse(input: String) -> Self::Res {
        let (b1, b2) = input
            .split_once("\n\n")
            .expect("TwoBlocks expects a \\n\\n in the input");
        (B1::parse(b1.to_owned()), B2::parse(b2.to_owned()))
    }
}

pub struct Ranges {}
impl InputParser for Ranges {
    type Res = Vec<(i64, i64)>;
    fn parse(input: String) -> Self::Res {
        Lines::parse(input)
            .into_iter()
            .map(|line| {
                let (part1, part2) = line
                    .split_once("-")
                    .expect("Ranges expects a \\d+-\\d+ format");
                (
                    part1
                        .parse::<i64>()
                        .expect("first part of range should be parsable as i64"),
                    part2
                        .parse::<i64>()
                        .expect("second part of range should be parsable as i64"),
                )
            })
            .collect_vec()
    }
}

pub struct CommaSeparated<T> {
    marker: PhantomData<T>,
}
impl<T: std::str::FromStr> InputParser for CommaSeparated<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    type Res = Vec<T>;
    fn parse(input: String) -> Self::Res {
        input
            .split(',')
            .map(|elem| {
                elem.parse::<T>()
                    .expect("CommaSeparated expects to be able to parse each item as T")
            })
            .collect_vec()
    }
}

pub struct ParsedLines<T: InputParser> {
    marker: PhantomData<T>,
}
impl<T: InputParser> InputParser for ParsedLines<T> {
    type Res = Vec<T::Res>;
    fn parse(input: String) -> Self::Res {
        Lines::parse(input)
            .into_iter()
            .map(|line| T::parse(line))
            .collect_vec()
    }
}

pub struct Numbers {}
impl InputParser for Numbers {
    type Res = Vec<i64>;
    fn parse(input: String) -> Self::Res {
        Lines::parse(input)
            .into_iter()
            .map(|line| {
                line.parse::<i64>()
                    .expect("Numbers expects all lines to be numeric")
            })
            .collect_vec()
    }
}

pub fn count_digits(mut x: i64) -> i64 {
    if x < 0 {
        panic!("Refusing to count digits of a negative number {x}");
    }
    if x == 0 {
        return 1;
    }
    let mut cnt = 0;
    while x != 0 {
        cnt += 1;
        x /= 10;
    }
    cnt
}

pub enum NeiDirs {
    BaseFour,
    Omni,
}

pub fn neis(x: i64, y: i64, coords: &CoordsResult, nei_dirs: NeiDirs) -> Vec<(i64, i64, char)> {
    let mut result = Vec::new();
    let base_four = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let omni = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
    let iter: Box<dyn Iterator<Item = (i64, i64)>> = match nei_dirs {
        NeiDirs::BaseFour => Box::new(base_four.into_iter()),
        NeiDirs::Omni => Box::new(base_four.into_iter().chain(omni.into_iter())),
    };
    for (xdelta, ydelta) in iter {
        let (xi, yi) = (x + xdelta, y + ydelta);
        coords.get(&xi).inspect(|col| {
            col.get(&yi).inspect(|val| {
                result.push((xi, yi, **val));
            });
        });
    }
    result
}

pub fn combine_digits(it: impl Iterator<Item = char>) -> Option<i64> {
    it.fold(None, |acc, ch| {
        Some(acc.unwrap_or(0) * 10 + (ch.to_digit(10).unwrap() as i64))
    })
}

fn get_cache_file() -> Option<PathBuf> {
    let part = var_or("PART", "2");
    let task_num = var_or("TASK_NUM", "1");

    let pwd = std::env::current_dir()
        .expect("current dir to be accessible")
        .canonicalize()
        .ok()?;
    let mut year_dir = if pwd.file_name().unwrap() == "src" {
        pwd.parent().expect("aoc source to not be in fs root")
    } else {
        &pwd
    }
    .to_owned();
    if year_dir.parent().unwrap().file_name().unwrap().to_str() != Some("adventofcode") {
        panic!("Not in a year dir, avoiding pollution of random files!");
    }
    year_dir.push("cache");
    let mut cache_dir = year_dir;
    std::fs::DirBuilder::new()
        .recursive(true)
        .create(&cache_dir)
        .unwrap();
    cache_dir.push(format!(
        "cache_{task_num}_part{part}_{}.json",
        if is_example() { "ex" } else { "real" }
    ));
    let cache_file = cache_dir;
    Some(cache_file)
}

pub fn get_cached_line_result(line: usize) -> Option<i64> {
    let cache_file = get_cache_file()?;
    println!("Looking up cache @ {}", cache_file.display());
    if !cache_file.exists() {
        return None;
    }
    let contents = std::fs::read(cache_file).unwrap();
    let cache = serde_json::from_slice(&contents).ok()?;
    match cache {
        Value::Object(m) => match m.get(&line.to_string())? {
            Value::Number(val) => {
                println!("Successfully found value {val} in cache");
                val.as_i64()
            }
            _ => None,
        },
        _ => None,
    }
}

pub fn save_line_result_to_cache(line: usize, val: i64) {
    let cache_file = get_cache_file().unwrap();
    println!(
        "Caching result {val} for line {line} to {}",
        cache_file.display()
    );
    if !cache_file.exists() {
        std::fs::write(&cache_file, "{}").unwrap();
    }
    let contents = std::fs::read(&cache_file).unwrap();
    let mut cache = serde_json::from_slice(&contents).unwrap();
    if let Value::Object(ref mut m) = cache {
        m.insert(line.to_string(), serde_json::json!(val));
    }
    std::fs::write(cache_file, serde_json::to_string(&cache).unwrap()).unwrap();
    println!("Successfully written cache {cache}");
}
