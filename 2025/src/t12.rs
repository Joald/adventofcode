#[allow(unused)]
use crate::prelude::*;
//fn rotate(x: (i64, i64)) -> (i64, i64){
//    (2 - x.1, x.0)
//}
//fn flip(x: (i64, i64)) -> (i64, i64) {
//    (2 - x.0, x.1)
//}
#[allow(unused)]
pub fn solve_12(part: usize, input: String) -> i64 {
    let mut blocks = Blocks::parse(input);
    let region_lines = blocks.pop().unwrap();
    let regions = region_lines
        .iter()
        .map(|line| {
            let (size, counts) = line.split_once(": ").unwrap();
            let (size_x, size_y) = size.split_once("x").unwrap();
            let counts = counts
                .split(" ")
                .map(|num| num.parse::<i64>().unwrap())
                .collect_vec();
            (
                size_x.parse::<i64>().unwrap(),
                size_y.parse::<i64>().unwrap(),
                counts,
            )
        })
        .collect_vec();
    let blocks = blocks
        .into_iter()
        .map(|block| {
            block
                .iter()
                .skip(1)
                .enumerate()
                .flat_map(|(i, line)| {
                    std::iter::repeat(i).zip(
                        line.chars()
                            .enumerate()
                            .filter(|(_, c)| *c == '#')
                            .map(|(j, _)| j),
                    )
                })
                .collect_vec()
        })
        .collect_vec();
    //dbg!(blocks, regions);

    enum Result {Good, Bad, Inconclusive};
    let results = regions.into_iter().map(|(size_x , size_y, region)|{
        // trivial check: if we can fix full 3x3s we can fit partial 3x3s
        if (size_x / 3) * (size_y / 3) >= region.iter().sum() {
            return Result::Good;
        }
        // trivial check: if we can't fit all the dots we can't fit rearranged dots
        if region.iter().enumerate().map(|(i, x)| {
            x * blocks[i].len() as i64
        }).sum::<i64>() > size_x * size_y{
            return Result::Bad;
        }

        // Block ID:
        Result::Inconclusive
    }).collect_vec();
    let (good, bad, inconclusive) = results.into_iter().fold((0, 0, 0), |(g, b, i), x| {match x {
        Result::Good => (g+1,b,i),
        Result::Bad => (g, b+1,i),
        Result::Inconclusive => (g, b,i+1)

        
    }});
    println!("Good: {good}, bad:{bad}, inconclusive: {inconclusive}");
    good

}
