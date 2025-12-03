use crate::prelude::*;

#[allow(unused)]
pub fn solve_03(part: usize, input: String) -> i64 {
    let lines = Lines::parse(input);
    let mut sum = 0;
    let mut mult = 1;

    let re = Regex::new(r"(?:mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

    for line in lines {
        println!("Line: \"{line}\"");
        for mtch in re.find_iter(&line) {
            dbg!(&mtch);
            let mtch: &str = mtch.into();
            match mtch {
                "do()" => mult = 1,
                "don't()" => {
                    if part == 2 {
                        mult = 0;
                    }
                }
                _ => {
                    let caps = re.captures(mtch).unwrap();
                    dbg!(&caps);
                    sum += caps[1].parse::<i64>().unwrap() * caps[2].parse::<i64>().unwrap() * mult;
                }
            }
        }
    }
    sum
}
