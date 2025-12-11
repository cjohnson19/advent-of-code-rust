use std::str::FromStr;

use advent_of_code::Coord;
use itertools::Itertools;

advent_of_code::solution!(9);

struct PartOneInput {
    coords: Vec<Coord>,
}

impl FromStr for PartOneInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PartOneInput {
            coords: s
                .lines()
                .map(|line| {
                    line.split_once(",")
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                        .unwrap()
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CoordLineType {
    Horizontal,
    Vertical,
}

impl CoordLineType {
    fn for_coords((x1, _): Coord, (x2, _): Coord) -> Self {
        if x1 == x2 {
            Self::Horizontal
        } else {
            Self::Vertical
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CoordSpan {
    lo: Coord,
    hi: Coord,
    ty: CoordLineType,
}

impl CoordSpan {
    fn new(c1: Coord, c2: Coord) -> Self {
        assert!(c1.0 == c2.0 || c1.1 == c2.1);
        let ty = CoordLineType::for_coords(c1, c2);
        if c1.0 < c2.0 || c1.1 < c2.1 {
            Self { lo: c1, hi: c2, ty }
        } else {
            Self { lo: c2, hi: c1, ty }
        }
    }

    fn merge<'a>(c1: CoordSpan, c2: CoordSpan) -> Box<dyn Iterator<Item = CoordSpan>> {
        match Self::extend(&c1, &c2) {
            Some(_) => Box::new(std::iter::once(c1)),
            None => Box::new([c1, c2].into_iter()),
        }
    }

    fn extend(c1: &CoordSpan, c2: &CoordSpan) -> Option<Self> {
        match (c1.ty, c2.ty) {
            (CoordLineType::Vertical, CoordLineType::Vertical) => {
                (c1.lo.0 <= c2.hi.0 && c1.hi.0 >= c2.lo.0).then(|| Self {
                    lo: (c1.lo.0.min(c2.lo.0), c1.lo.1),
                    hi: (c1.hi.0.max(c2.hi.0), c1.hi.1),
                    ty: CoordLineType::Vertical,
                })
            }
            (CoordLineType::Horizontal, CoordLineType::Horizontal) => {
                (c1.lo.1 <= c2.hi.1 && c1.hi.1 >= c2.lo.1).then(|| Self {
                    lo: (c1.lo.0, c1.lo.1.min(c2.lo.1)),
                    hi: (c2.hi.0, c1.hi.1.max(c2.hi.1)),
                    ty: CoordLineType::Horizontal,
                })
            }
            (CoordLineType::Vertical, CoordLineType::Horizontal)
            | (CoordLineType::Horizontal, CoordLineType::Vertical) => None,
        }
    }

    fn contains(&self, p: &Coord) -> bool {
        todo!()
        // match self.ty {
        //     CoordLineType::Horizontal => (self.lo.1..self.lo.0),
        //     CoordLineType::Vertical => todo!(),
        // }
    }
}

fn internal_squares(c1: Coord, c2: Coord) -> impl Iterator<Item = (usize, usize)> {
    let start_x = c1.0.min(c2.0);
    let end_x = c1.0.max(c2.0);
    let start_y = c1.1.min(c2.1);
    let end_y = c1.1.max(c2.1);

    (start_x..=end_x).flat_map(move |x| (start_y..=end_y).map(move |y| (x, y)))
}

struct PartTwoInput {
    red_squares: Vec<Coord>,
    green_borders: Vec<CoordSpan>,
}

impl PartTwoInput {
    fn inside_shape(&self, coord: &Coord) -> bool {
        // let mut wall_count = 0;
        // for x in (0..coord.0) {
        //     if self.green_borders.iter().any(|s| )

        // }
        todo!()
    }
}

impl FromStr for PartTwoInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<Coord> = s
            .lines()
            .map(|line| {
                line.split_once(",")
                    .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                    .unwrap()
            })
            .collect();
        let borders: Vec<CoordSpan> = coords
            .iter()
            .circular_tuple_windows()
            .map(|(x, y)| CoordSpan::new(*x, *y))
            .tuple_combinations()
            .flat_map(|(x, y)| CoordSpan::merge(x, y))
            .collect();

        Ok(Self {
            red_squares: coords,
            green_borders: borders,
        })
    }
}

fn area(c1: &Coord, c2: &Coord) -> usize {
    (c1.0.abs_diff(c2.0) + 1) * (c1.1.abs_diff(c2.1) + 1)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input: PartOneInput = input.parse().unwrap();
    input
        .coords
        .iter()
        .tuple_combinations()
        .map(|(a, b)| area(a, b) as u64)
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input: PartTwoInput = input.parse().unwrap();
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_area() {
        let a = area(&(2, 5), &(11, 1));
        assert_eq!(a, 50);
    }

    #[test]
    fn test_p1_simple() {
        let a = part_one("2,5\n11,1");
        assert_eq!(a, Some(50));
    }

    #[test]
    fn extend_test() {
        // vertical
        let span1 = CoordSpan::new((0, 5), (0, 20));
        let span2 = CoordSpan::new((0, 15), (0, 50));
        assert_eq!(
            CoordSpan::extend(&span1, &span2),
            Some(CoordSpan::new((0, 5), (0, 50)))
        );
        // horizontal
        let span1 = CoordSpan::new((5, 0), (20, 0));
        let span2 = CoordSpan::new((15, 0), (50, 0));
        assert_eq!(
            CoordSpan::extend(&span1, &span2),
            Some(CoordSpan::new((5, 0), (50, 0)))
        );
        // Vertical v. horizontal
        let span1 = CoordSpan::new((0, 5), (0, 20));
        let span2 = CoordSpan::new((15, 0), (50, 0));
        assert_eq!(CoordSpan::extend(&span1, &span2), None,);
        // non overlapping
        let span1 = CoordSpan::new((5, 0), (20, 0));
        let span2 = CoordSpan::new((21, 0), (50, 0));
        assert_eq!(CoordSpan::extend(&span1, &span2), None,);
    }
}
