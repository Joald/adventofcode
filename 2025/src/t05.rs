use crate::prelude::*;

#[allow(unused)]
pub fn solve_05(part: usize, input: String) -> i64 {
    let (ranges, numbers) = TwoBlocks::<Ranges, Numbers>::parse(input);

    if part == 1 {
        numbers.into_iter().fold(0, |acc, x| {
            acc + if ranges.iter().any(|(beg, end)| *beg <= x && x <= *end) {
                1
            } else {
                0
            }
        })
    } else {
        let mut numbers: HashSet<(i64, i64)> = HashSet::new();

        for (beg, end) in ranges.into_iter() {
            let coinciding = numbers.clone().into_iter().filter(|(x, y)| {
                *x <= beg && beg <= *y
                    || *x <= end && end <= *y
                    || beg <= *x && *x <= end
                    || beg <= *y && *y <= end
            });
            let new_range = coinciding.fold((beg, end), |(accx, accy), (x, y)| {
                numbers.remove(&(x, y));
                (accx.min(x), accy.max(y))
            });
            numbers.insert(new_range);
        }
        numbers.into_iter().fold(0, |acc, (x, y)| acc + y - x + 1)
    }
}
