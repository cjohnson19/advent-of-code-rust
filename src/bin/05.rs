use std::{ops::RangeInclusive, str::FromStr};

advent_of_code::solution!(5);

struct Input {
    ranges: Vec<RangeInclusive<usize>>,
    ranges_raw: Vec<(usize, usize)>,
    ingredients: Vec<usize>,
}

impl Input {
    fn in_range(&self, i: usize) -> bool {
        self.ranges.iter().any(|e| e.contains(&i))
    }
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ranges, ingredients) = s.split_once("\n\n").unwrap();
        let ranges_raw: Vec<_> = ranges
            .lines()
            .map(|l| {
                let (l, h) = l.split_once("-").unwrap();
                (l.parse().unwrap(), h.parse().unwrap())
            })
            .collect();
        let ranges = ranges_raw.iter().map(|(l, h)| *l..=*h).collect();
        let ingredients = ingredients.lines().map(|c| c.parse().unwrap()).collect();

        Ok(Self {
            ranges,
            ranges_raw,
            ingredients,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Input = input.parse().unwrap();

    Some(
        input
            .ingredients
            .iter()
            .filter(|i| input.in_range(**i))
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Input = input.parse().unwrap();

    let mut ranges = input.ranges_raw;
    ranges.sort_by(|(l1, _), (l2, _)| l1.cmp(l2));

    let mut it = ranges.iter().peekable();
    let mut ans = 0;

    while let Some((l, h)) = it.next() {
        let mut l = l;
        let mut h = h;
        while let Some((l2, h2)) = it.peek()
            && l <= h2
            && h >= l2
        {
            l = l2.min(l);
            h = h2.max(h);
            it.next();
        }
        ans += h - l + 1
    }

    Some(ans as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
