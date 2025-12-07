use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use advent_of_code::Coord;

advent_of_code::solution!(7);

struct GridInput {
    start: Coord,
    splitters: Vec<HashSet<usize>>,
    height: usize,
}

impl FromStr for GridInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let n = lines.len();
        let m = lines[0].len();
        let mut start = (0, 0);
        let mut splitters = vec![HashSet::new(); n];
        for i in 0..n {
            for j in 0..m {
                if lines[i][j] == 'S' {
                    start = (i, j)
                } else if lines[i][j] == '^' {
                    splitters[i].insert(j);
                }
            }
        }

        Ok(Self {
            start,
            splitters,
            height: n,
        })
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let g: GridInput = input.parse().unwrap();
    let mut beams: HashSet<usize> = [g.start.1].into_iter().collect();
    let mut ans = 0u64;
    for row in (g.start.0 + 1)..g.height {
        let (new_beams, old_beams): (HashSet<usize>, HashSet<usize>) =
            beams.iter().partition(|col| g.splitters[row].contains(col));
        ans += new_beams.len() as u64;
        beams = old_beams.union(&new_beams
            .iter()
            .flat_map(|col| vec![col - 1, col + 1])
            .collect()
        ).cloned().collect()
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let g: GridInput = input.parse().unwrap();
    let mut beams: HashMap<usize, usize> = [(g.start.1, 1)].into_iter().collect();
    for row in (g.start.0 + 1)..g.height {
        let (split_beams, mut unsplit_beams): (HashMap<usize, usize>, HashMap<usize, usize>) =
            beams
                .iter()
                .partition(|(col, _)| g.splitters[row].contains(col));
        for (new_beam, c) in split_beams {
            unsplit_beams
                .entry(new_beam - 1)
                .and_modify(|v| *v += c)
                .or_insert(c);
            unsplit_beams
                .entry(new_beam + 1)
                .and_modify(|v| *v += c)
                .or_insert(c);
        }
        beams = unsplit_beams;
    }

    Some(beams.values().sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
