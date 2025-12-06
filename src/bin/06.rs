use std::str::FromStr;

use advent_of_code::transpose;
use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Debug)]
enum Op {
    Mul,
    Add,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "*" => Self::Mul,
            "+" => Self::Add,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug)]
struct Input {
    numbers: Vec<Vec<u64>>,
    operations: Vec<Op>,
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (nums, ops) = s.trim_end().rsplit_once('\n').unwrap();
        let nums: Vec<Vec<_>> = nums
            .lines()
            .map(|l| l.split_whitespace().map(|n| n.parse().unwrap()).collect())
            .collect();
        let ops = ops
            .split_whitespace()
            .map(|o| Op::from_str(o).unwrap())
            .collect();
        Ok(Self {
            numbers: nums,
            operations: ops,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: Input = input.parse().unwrap();
    let op_nums = transpose(input.numbers);
    Some(
        input
            .operations
            .into_iter()
            .zip(op_nums.into_iter())
            .map(|(op, nums)| {
                nums.into_iter()
                    .reduce(|l, r| match op {
                        Op::Mul => l * r,
                        Op::Add => l + r,
                    })
                    .unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (number_part, ops_line) = input.trim_end().rsplit_once('\n').unwrap();

    let ops: Vec<Op> = ops_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let c: Vec<Vec<char>> = number_part
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let s: String = transpose(c)
        .into_iter()
        .map(|col| col.iter().collect::<String>().trim().to_string())
        .join("\n");

    Some(
        s.split("\n\n")
            .zip(&ops)
            .map(|(nums_str, op)| {
                nums_str
                    .split_whitespace()
                    .map(|n| n.parse::<u64>().unwrap())
                    .reduce(|l, r| match op {
                        Op::Mul => l * r,
                        Op::Add => l + r,
                    })
                    .unwrap()
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
