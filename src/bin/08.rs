use core::fmt;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashSet},
    hash::Hash,
    str::FromStr,
};

advent_of_code::solution!(8);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Coord3D {
    x: isize,
    y: isize,
    z: isize,
}

impl fmt::Display for Coord3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl FromStr for Coord3D {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let &[x, y, z] = s
            .split(",")
            .map(|p| p.parse::<isize>().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
        {
            Ok(Self { x, y, z })
        } else {
            Err("Not a valid 3d coordinate".to_owned())
        }
    }
}

impl Coord3D {
    fn distance(&self, other: &Self) -> f64 {
        let x = (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        (x as f64).sqrt()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Forest {
    parents: BTreeMap<Coord3D, Coord3D>,
    pub graphs: BTreeMap<Coord3D, BTreeSet<Coord3D>>,
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (node, parent) in self.parents.iter() {
            writeln!(f, "{node} -> {parent}")?;
        }
        writeln!(f)?;

        let mut seen_parents = BTreeSet::new();
        for (n, g) in self.graphs.iter() {
            let p = self.get_parent(*n);
            if !seen_parents.contains(&p) {
                write!(f, "{p} -- ")?;
                for node in g.iter() {
                    write!(f, "{node},")?;
                }
                writeln!(f)?;
                seen_parents.insert(p);
            }
        }

        Ok(())
    }
}

impl Forest {
    fn new(nodes: &[Coord3D]) -> Self {
        Self {
            parents: BTreeMap::new(),
            graphs: nodes
                .iter()
                .map(|c| (*c, vec![*c].into_iter().collect()))
                .collect(),
        }
    }

    fn get_parent(&self, c: Coord3D) -> Coord3D {
        let mut parent = c;
        while let Some(p) = self.parents.get(&parent) {
            parent = *p;
        }
        parent
    }

    fn union(&mut self, c1: Coord3D, c2: Coord3D) {
        let c1_parent = self.get_parent(c1);
        let c2_parent = self.get_parent(c2);
        let c1_graph = self.graphs.get(&c1_parent).unwrap();
        let c2_graph = self.graphs.get(&c2_parent).unwrap();
        let new_set: BTreeSet<Coord3D> = c1_graph.union(c2_graph).copied().collect();
        self.parents.insert(c2_parent, c1);
        self.graphs.insert(c1_parent, new_set.clone());
        self.graphs.remove(&c2_parent);
    }

    fn connected(&self, c1: Coord3D, c2: Coord3D) -> bool {
        self.get_parent(c2) == self.get_parent(c1)
    }

    fn graph_count(&self) -> usize {
        let mut seen_parents = HashSet::new();
        let mut graph_sizes = Vec::new();
        for (n, _) in self.graphs.iter() {
            let p = self.get_parent(*n);
            if !seen_parents.contains(&p) {
                graph_sizes.push(self.graphs.get(&p).unwrap().len());
                seen_parents.insert(p);
            }
        }
        graph_sizes.sort_by(|a, b| b.cmp(a));

        graph_sizes.iter().take(3).product()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CoordPair(Coord3D, Coord3D);

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for CoordPair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0
            .distance(&self.1)
            .partial_cmp(&other.0.distance(&other.1))
    }
}

impl Ord for CoordPair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0
            .distance(&self.1)
            .total_cmp(&other.0.distance(&other.1))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords: Vec<Coord3D> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut g = Forest::new(&coords);
    let mut q = BinaryHeap::new();
    let mut measured: HashSet<(usize, usize)> = (0..coords.len()).map(|i| (i, i)).collect();
    for i in 0..coords.len() {
        for j in 0..coords.len() {
            if !(measured.contains(&(i, j)) || measured.contains(&(j, i))) {
                q.push(Reverse(CoordPair(coords[i], coords[j])));
                measured.insert((i, j));
            }
        }
    }
    for _ in 0..1000 {
        while let Some(Reverse(CoordPair(c1, c2))) = q.pop() {
            if !g.connected(c1, c2) {
                g.union(c1, c2);
            }
        }
    }

    Some(g.graph_count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords: Vec<Coord3D> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut g = Forest::new(&coords);
    let mut q = BinaryHeap::new();
    let mut measured: HashSet<(usize, usize)> = (0..coords.len()).map(|i| (i, i)).collect();
    for i in 0..coords.len() {
        for j in 0..coords.len() {
            if !(measured.contains(&(i, j)) || measured.contains(&(j, i))) {
                q.push(Reverse(CoordPair(coords[i], coords[j])));
                measured.insert((i, j));
            }
        }
    }
    let mut last = None;
    while let Some(Reverse(CoordPair(c1, c2))) = q.pop() {
        if !g.connected(c1, c2) {
            g.union(c1, c2);
            last = Some((c1, c2));
        }
    }

    last.map(|(c1, c2)| (c1.x * c2.x) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
