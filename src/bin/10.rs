use core::fmt;
use std::{fmt::format, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Machine {
    lights: u32,
    switches: Vec<u32>,
    joltage: (),
}

fn parse_switch(s: &str) -> u32 {
    s.strip_prefix("(")
        .unwrap()
        .strip_suffix(")")
        .unwrap()
        .split(",")
        .map(|idx| idx.parse::<usize>().unwrap())
        .fold(0, |a, i| a | (1 << i))
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let light_str = parts.next().unwrap();
        let binary_str = light_str
            .strip_prefix("[")
            .unwrap()
            .strip_suffix("]")
            .unwrap()
            .chars()
            .rev()
            .map(|a| if a == '#' { '1' } else { '0' })
            .collect::<String>();
        let lights = u32::from_str_radix(&binary_str, 2).unwrap();

        let switches: Vec<u32> = parts
            .take_while(|s| s.starts_with("("))
            .map(|switch_str| parse_switch(switch_str))
            .collect();
        Ok(Self {
            lights,
            switches,
            joltage: (),
        })
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:b}", self.lights)?;
        for switch in &self.switches {
            write!(f, " {switch:b}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Machines {
    machines: Vec<Machine>,
}

impl FromStr for Machines {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let machines = s.lines().map(Machine::from_str).collect::<Result<_, _>>()?;
        Ok(Self { machines })
    }
}

impl fmt::Display for Machines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for machine in &self.machines {
            writeln!(f, "{}", machine)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines: Machines = input.parse().unwrap();
    machines
        .machines
        .iter()
        .map(|machine| {
            for i in 1..=machine.switches.len() {
                for combo in machine.switches.iter().combinations(i) {
                    let switch_res = combo.iter().fold(0, |acc, val| acc ^ **val);
                    if switch_res == machine.lights {
                        return Some(i as u64);
                    }
                }
            }
            None
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_switch() {
        let n = parse_switch("(1,2,4)");
        assert_eq!(n, 0b10110)
    }

    #[test]
    fn test_p1() {
        assert_eq!(
            part_one("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            Some(3)
        );
    }
}
