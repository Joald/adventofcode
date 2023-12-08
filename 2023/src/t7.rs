use crate::dbg::dprintln;
use itertools::Itertools;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
enum Set {
    Single,
    Pair,
    TwoPair,
    Three,
    Full,
    Four,
    Five,
}

fn parse_set(part: i64, hand: &[i64]) -> Set {
    let hand: Vec<_> = hand.iter().filter(|c| **c != 1).collect();

    let js = 5 - hand.len();

    let exists_with_cnt = |num| {
        hand.iter()
            .any(|card| hand.iter().filter(|other| *other == card).count() == num)
    };
    let count_of_exist_with_cnt = |num| {
        hand.iter()
            .filter(|card| hand.iter().filter(|other| *other == *card).count() == 2)
            .count()
    };
    if part == 1 {
        if hand.iter().all_equal() {
            Set::Five
        } else if exists_with_cnt(4) {
            Set::Four
        } else if exists_with_cnt(3) && exists_with_cnt(2) {
            Set::Full
        } else if exists_with_cnt(3) {
            Set::Three
        } else if count_of_exist_with_cnt(2) == 4 {
            Set::TwoPair
        } else if exists_with_cnt(2) {
            Set::Pair
        } else if hand.iter().all_unique() {
            Set::Single
        } else {
            panic!("impossibru")
        }
    } else {
        if hand.iter().all_equal() {
            Set::Five
        } else if exists_with_cnt(4)
            || exists_with_cnt(3) && js == 1
            || exists_with_cnt(2) && js == 2
            || js == 3
        {
            Set::Four
        } else if exists_with_cnt(3) && exists_with_cnt(2)
            || count_of_exist_with_cnt(2) == 4 && js == 1
        {
            Set::Full
        } else if exists_with_cnt(3) || exists_with_cnt(2) && js == 1 || js == 2 {
            Set::Three
        } else if count_of_exist_with_cnt(2) == 4 {
            Set::TwoPair
        } else if exists_with_cnt(2) || js == 1 {
            Set::Pair
        } else if hand.iter().all_unique() {
            Set::Single
        } else {
            panic!("impossibru")
        }
    }
}

#[allow(unused)]
pub fn solve_7(part: usize, lines: Vec<String>) -> i64 {
    lines
        .into_iter()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let (hand, bet) = line.split_once(" ").unwrap();
            let cards: Vec<_> = hand
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 1,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c as i64 - '0' as i64,
                })
                .collect();
            let set = parse_set(part as i64, &cards);
            (cards, set, bet.parse::<i64>().unwrap())
        })
        .sorted_by_cached_key(|(hand, set, bet)| (set.clone(), hand.clone()))
        .enumerate()
        .map(|(i, hsb)| {
            let res = (i + 1) as i64 * hsb.2;
            dprintln!("i={}, hsb={:?} -> {}", i, hsb, res);
            res
        })
        .sum()
}
