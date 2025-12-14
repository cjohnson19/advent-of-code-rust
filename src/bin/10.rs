use core::fmt;
use std::{iter::Sum, str::FromStr};

use itertools::Itertools;
use z3::{Optimize, ast::Int};

advent_of_code::solution!(10);

#[derive(Debug, Clone)]
struct Machine {
    lights: u32,
    switches: Vec<u32>,
    switches_raw: Vec<Vec<usize>>,
    joltage: Vec<u32>,
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

        let switches_raw: Vec<Vec<_>> = parts
            .clone()
            .take_while(|s| s.starts_with("("))
            .map(|elem| {
                elem.strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap()
                    .split(",")
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        let switches = switches_raw
            .iter()
            .map(|raw_val| raw_val.iter().fold(0, |a, i| a | (1 << i)))
            .collect();

        let joltage: Vec<u32> = parts
            .last()
            .unwrap()
            .strip_prefix("{")
            .unwrap()
            .strip_suffix("}")
            .unwrap()
            .split(",")
            .map(|i| i.parse::<u32>().unwrap())
            .collect();
        Ok(Self {
            lights,
            switches,
            switches_raw,
            joltage,
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
    let machines: Machines = input.parse().unwrap();
    let mut ans = 0u64;
    for machine in machines.machines {
        let o = Optimize::new();
        let switch_flip_counts: Vec<_> = (0..machine.switches.len())
            .map(|i| Int::new_const(format!("flip_{i}")))
            .collect();
        for switch_flip in &switch_flip_counts {
            o.assert(&switch_flip.ge(0));
        }
        for idx in 0..machine.joltage.len() {
            let r = z3::ast::Int::sum(
                machine
                    .switches_raw
                    .iter()
                    .enumerate()
                    .filter(|(_, s)| s.contains(&idx))
                    .map(|(i, _)| switch_flip_counts[i].clone()),
            )
            .eq(machine.joltage[idx]);
            o.assert(&r);
        }
        o.minimize(&z3::ast::Int::sum(switch_flip_counts.iter()));
        o.check(&[]);
        let model = o.get_model().unwrap();
        let a = model.eval(&z3::ast::Int::sum(switch_flip_counts.iter()), true);
        ans += a.unwrap().as_u64().unwrap();
    }

    Some(ans)
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
        assert_eq!(result, Some(33));
    }

    #[test]
    fn test_p1() {
        assert_eq!(
            part_one("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            Some(3)
        );
    }
}
