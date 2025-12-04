use std::{collections::HashSet, fmt, str::FromStr};

use advent_of_code::all_neighbors;

advent_of_code::solution!(4);

struct Grid {
    taken: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.taken.contains(&(j, i)) {
                        "@"
                    } else {
                        "."
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<_> = s.lines().collect();
        let height = rows.len();
        let width = rows[0].len();

        let taken = s
            .lines()
            .enumerate()
            .flat_map(|(i, l)| {
                l.char_indices()
                    .filter_map(|(j, c)| (c == '@').then_some((j, i)))
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();

        Ok(Self {
            taken,
            width,
            height,
        })
    }
}

fn num_tp_neighbors(g: &Grid, x: usize, y: usize) -> u64 {
    all_neighbors((x, y), g.width, g.height)
        .filter(|(x, y)| g.taken.contains(&(*x, *y)))
        .count() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let g: Grid = input.parse().unwrap();
    let mut ans = 0;
    for i in 0..g.height {
        for j in 0..g.width {
            if g.taken.contains(&(j, i)) && num_tp_neighbors(&g, j, i) < 4 {
                ans += 1;
            }
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut g: Grid = input.parse().unwrap();
    let mut ans = 0;
    loop {
        let start_tp_count = g.taken.len();
        for i in 0..g.height {
            for j in 0..g.width {
                if g.taken.contains(&(j, i)) && num_tp_neighbors(&g, j, i) < 4 {
                    ans += 1;
                    g.taken.remove(&(j, i));
                }
            }
        }
        if start_tp_count == g.taken.len() {
            break;
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
