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

// DP solution for arbitrary `m` numbers to include
fn max_joltage_m(b: &BatteryBank, m: usize) -> u64 {
    let n = b.0.len();
    let mut dp = vec![vec![0u64; n + 1]; m + 1];
    // initial, fill out the basic portion of the table where we take the first
    // `m` digits and call that good.
    for i in 1..=m {
        for j in i..=n {
            dp[i][j] = dp[i - 1][j - 1] * 10 + b.0[j - 1];
        }
    }

    // Now consider changing some digits around
    for i in 2..=m {
        for j in (i + 1)..=n {
            // Take the best joltage from the previous version which did not
            // rely on this number and add on this one
            let choose_this_digit = dp[i - 1][j - 1] * 10 + b.0[j - 1];
            // The best one we've seen so far
            let keep_previous_best = dp[i][j - 1];
            // The best one we've seen so far, but replace the last digit with
            // this one.
            let remove_previous_digit = remove_last_digit(dp[i][j - 1]) + b.0[j - 1];
            // Whatever solution we currently have
            let current = dp[i][j];
            // Our new best here is the maximum of all of those possible choices
            dp[i][j] = choose_this_digit
                .max(keep_previous_best)
                .max(current)
                .max(remove_previous_digit);
        }
    }

    dp[m][dp[0].len() - 1]
}

fn remove_last_digit(n: u64) -> u64 {
    n / 10 * 10
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

    #[test]
    fn remove_last() {
        assert_eq!(remove_last_digit(13), 10);
    }
}
