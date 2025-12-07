use crate::prelude::*;
pub fn solve_07(part: usize, input: String) -> i64 {
    let mut coords = Coords::parse(input);
    let mut cnt = 0;
    let mut numbers: HashMap<i64, HashMap<i64, i64>> =
        HashMap::from_iter(coords.clone().into_iter().map(|(k, v)| {
            (
                k,
                v.into_iter()
                    .map(|(k, v)| (k, if v == 'S' { 1 } else { 0 }))
                    .collect(),
            )
        }));

    for x in 0..(coords.len() as i64) - 1 {
        for y in 0..coords[&x].len() as i64 {
            let c = coords[&x][&y];
            let count = numbers[&x][&y];
            if c == 'S' || c == '|' {
                if coords[&(x + 1)][&y] == '^' {
                    if y > 0
                        && let Some(row) = coords.get_mut(&(x + 1))
                    {
                        row.insert(y - 1, '|');
                        *numbers
                            .get_mut(&(x + 1))
                            .unwrap()
                            .get_mut(&(y - 1))
                            .unwrap() += count;
                    }
                    if y < coords[&x].len() as i64 - 1
                        && let Some(row) = coords.get_mut(&(x + 1))
                    {
                        row.insert(y + 1, '|');
                        *numbers
                            .get_mut(&(x + 1))
                            .unwrap()
                            .get_mut(&(y + 1))
                            .unwrap() += count;
                    }
                    cnt += 1;
                } else if let Some(row) = coords.get_mut(&(x + 1)) {
                    row.insert(y, '|');

                    *numbers.get_mut(&(x + 1)).unwrap().get_mut(&y).unwrap() += count;
                }
            }
        }
    }
    if part == 1 {
        cnt
    } else {
        numbers[&(numbers.len() as i64 - 1)].values().sum()
    }
}
