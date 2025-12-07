use std::{ops::Range, str::FromStr};

advent_of_code::solution!(2);

pub struct IDRange {
    low: String,
    high: String,
}

impl FromStr for IDRange {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("-").map_or_else(
            || Err("Needs Dash".to_owned()),
            |(l, h)| {
                Ok(Self {
                    low: l.to_string(),
                    high: h.to_string(),
                })
            },
        )
    }
}

impl From<IDRange> for Range<usize> {
    fn from(value: IDRange) -> Self {
        Self {
            start: value.low.parse().unwrap(),
            end: value.high.parse::<usize>().unwrap() + 1,
        }
    }
}

fn parse_input(input: &str) -> Vec<IDRange> {
    input
        .split(",")
        .map(IDRange::from_str)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn is_repetition(n: usize) -> bool {
    let s = n.to_string();
    let l = s.len();
    if l % 2 == 1 {
        return false;
    }
    s[..l / 2] == s[l / 2..]
}

fn is_n_repetitions(n: usize) -> bool {
    let s = n.to_string();
    let l = s.len();
    // look at all subsequences up to the halfway point
    for split_point in 1..=l / 2 {
        // post-solution adjustment - faster since we are only operating over
        // string slices, but it's much harder to read
        let chunk = &s[..split_point];
        let mut i = 0;
        let mut is_rep = true;
        // iterate through chunks of size `split_point`
        while i < l {
            // if `i + split_point > l` then the string is not a perfect series
            // of chunks size `split_point`, so it is not a repetition of the
            // subsequence.
            if i + split_point > l || &s[i..i + split_point] != chunk {
                is_rep = false;
                break;
            }
            i += split_point;
        }
        if is_rep {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .into_iter()
            .map(Into::<Range<usize>>::into)
            .flat_map(|r| r.filter_map(|n| is_repetition(n).then_some(n as u64)))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input(input)
            .into_iter()
            .map(Into::<Range<usize>>::into)
            .flat_map(|r| r.filter_map(|n| is_n_repetitions(n).then_some(n as u64)))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_one_simple() {
        let result = part_one("11-22");
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }

    #[test]
    fn is_rep_test() {
        assert!(is_repetition(11));
        assert!(!is_repetition(13));
        assert!(is_repetition(2222));
        assert!(is_repetition(123123));
    }

    #[test]
    fn is_rep_n_test() {
        assert!(is_n_repetitions(22));
        assert!(is_n_repetitions(565656));
    }
}
