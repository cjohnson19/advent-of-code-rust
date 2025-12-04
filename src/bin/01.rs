use std::str::FromStr;

advent_of_code::solution!(1);

#[derive(Debug, Clone, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Dir::Left),
            "R" => Ok(Dir::Right),
            _ => Err(format!("Invalid direction: {s}")),
        }
    }
}

#[derive(Debug, Clone)]
struct Move(Dir, i64);

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|line| {
            let (dir, steps) = line.split_at(1);
            Move(dir.parse().unwrap(), steps.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut ans = 0;
    let mut pos = 50;
    for Move(dir, n) in parse_moves(input) {
        let n = n % 100;
        // We only care about the final position, so I adjust leftward movement
        // into rightward, then modulo it back. This way, I don't have to deal
        // with negative numbers
        let n = if dir == Dir::Left { 100 - n } else { n };
        pos = (pos + n) % 100;
        if pos == 0 {
            ans += 1;
        }
    }

    Some(ans)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ans: u64 = 0;
    let mut pos = 50;
    for Move(dir, n) in parse_moves(input) {
        // add one for every time it would have overlapped by rotating 360 deg
        ans += (n as u64) / 100;
        let n = n % 100;
        // prevent double counting if n is multiple of 100.
        if n == 0 {
            continue;
        }
        match dir {
            Dir::Left => {
                let new_pos = pos + (100 - n);
                if new_pos <= 100 && pos != 0 {
                    // new_pos may be "negative", but we don't count it as another
                    // pass over 0 if the previous motion ended at 0
                    ans += 1;
                }
                pos = new_pos;
            }
            Dir::Right => {
                pos += n;
                // If pos is greater than or equal to 100, it passed over zero at some point
                if pos >= 100 {
                    ans += 1;
                }
            }
        }
        pos %= 100;
    }

    Some(ans)
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
        assert_eq!(result, Some(6));
    }
}
