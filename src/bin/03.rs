use std::{ops::Deref, str::FromStr};

advent_of_code::solution!(3);

#[derive(Debug, Clone)]
struct BatteryBank(Vec<u64>);

impl FromStr for BatteryBank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars().map(|c| c.to_digit(10).unwrap().into()).collect(),
        ))
    }
}

impl Deref for BatteryBank {
    type Target = Vec<u64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
struct Input(Vec<BatteryBank>);

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(BatteryBank::from_str)
                .collect::<Result<_, _>>()?,
        ))
    }
}

// Greedy algorithm. Take the highest tens place if available and if there are
// still digits remaining to take.
fn max_joltage(b: &BatteryBank) -> u64 {
    let mut tens = u64::MIN;
    let mut ones = u64::MAX;
    let mut iter = b.iter().peekable();
    while let Some(digit) = iter.next() {
        if *digit > tens && iter.peek().is_some() {
            tens = *digit;
            ones = u64::MIN;
        } else if *digit > ones {
            ones = *digit;
        }
    }
    format!("{tens}{ones}").parse().unwrap()
}

/// Greedy solution for arbitrary `m` numbers to include
///
/// The key insight is, if we want `m` characters, we have to include at least
/// the last `m`. However, for the remaining portion at the start, we can
/// greedily choose the largest number (which occurs first). Let
///
/// Consider a number of length `7` like:
///    8909127
/// where we want 3 top numbers. We have to reserve the last 2 and pick the
/// largest number from the other portion.
///    8909127
///         ^^ Reserved, cannot pick from
///    ----    Want the largest number
/// The first time, we find the first 9 to be the best, and permit looking at
/// one more digit
///    _909127
///          ^ Reserved
/// Next time, we find another 9
///    _9_9127
///            None Reserved
/// Finally, we search the remainder and find the 7
///    _9_9__7
/// 
/// ```
/// assert_eq!(max_joltage_m("8909127".parse().unwrap()), 997);
/// ```
///
/// Note, this is actually the general version of what I did for [`max_joltage`]
fn max_joltage_m(b: &BatteryBank, m: usize) -> u64 {
    let n = b.len();
    let mut m = m;
    let mut cur_best_idx = 0;
    let mut res = 0;
    while m > 0 {
        for i in (cur_best_idx + 1)..=(n - m) {
            if b[i] > b[cur_best_idx] {
                cur_best_idx = i;
            }
        }
        res = (res * 10) + b[cur_best_idx];
        cur_best_idx += 1;
        m -= 1;
    }
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Input = input.parse().unwrap();
    Some(input.0.iter().map(max_joltage).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: Input = input.parse().unwrap();
    Some(input.0.iter().map(|b| max_joltage_m(b, 12)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }

    #[test]
    fn test_part_two_single() {
        let b: BatteryBank = "234234".parse().unwrap();
        assert_eq!(max_joltage_m(&b, 3), 434);
    }
}
