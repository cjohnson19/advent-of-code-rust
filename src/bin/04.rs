use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    str::FromStr,
};

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
        let rows = s.lines().collect::<Vec<_>>();
        let height = rows.len();
        let width = rows[0].len();

        let taken = rows
            .iter()
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
    let g = input.parse::<Grid>().unwrap();
    Some(
        g.taken
            .iter()
            .filter(|(j, i)| num_tp_neighbors(&g, *j, *i) < 4)
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let g: Grid = input.parse().unwrap();
    let mut count_map = HashMap::new();
    let mut queue = VecDeque::new();
    let mut ans = 0;

    for (j, i) in &g.taken {
        let neighbors = num_tp_neighbors(&g, *j, *i);
        if neighbors < 4 {
            queue.push_back((*j, *i));
        }
        count_map.insert((*j, *i), neighbors);
    }

    while let Some((j, i)) = queue.pop_front() {
        ans += 1;
        for neighbor in all_neighbors((j, i), g.width, g.height) {
            count_map.entry(neighbor).and_modify(|e| {
                *e = *e - 1;
                if *e == 3 {
                    queue.push_back(neighbor);
                }
            });
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
