use std::collections::HashMap;

advent_of_code::solution!(11);

struct ServerRack<'a> {
    graph: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> ServerRack<'a> {
    fn parse(s: &'a str) -> Self {
        Self {
            graph: s
                .lines()
                .map(|line| {
                    let (input, output_parts) = line.split_once(":").unwrap();
                    let output_strs = output_parts.split_whitespace().collect();
                    (input, output_strs)
                })
                .collect(),
        }
    }

    fn outputs(&self, n: &str) -> &[&'a str] {
        self.graph.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
}

pub fn part_one<'a>(input: &'a str) -> Option<u64> {
    let g: ServerRack<'a> = ServerRack::parse(input);
    let mut dp: HashMap<&'a str, usize> = HashMap::new();

    // no need to track seen nodes, there are no cycles
    fn dfs<'a>(g: &ServerRack<'a>, node: &'a str, dp: &mut HashMap<&'a str, usize>) -> usize {
        if let Some(a) = dp.get(node) {
            *a
        } else if node == "out" {
            1
        } else {
            let pathways = g
                .outputs(node)
                .iter()
                .map(|neighbor| dfs(g, neighbor, dp))
                .sum::<usize>();
            dp.insert(node, pathways);
            pathways
        }
    }

    let ans = dfs(&g, "you", &mut dp);
    Some(ans as u64)
}

pub fn part_two<'a>(input: &'a str) -> Option<u64> {
    let g: ServerRack<'a> = ServerRack::parse(input);
    let mut pathways: HashMap<(&'a str, bool, bool), usize> = HashMap::new();

    fn dfs<'a>(
        g: &ServerRack<'a>,
        node: &'a str,
        seen_dac: bool,
        seen_fft: bool,
        dp: &mut HashMap<(&'a str, bool, bool), usize>,
    ) -> usize {
        if let Some(a) = dp.get(&(node, seen_dac, seen_fft)) {
            *a
        } else if node == "out" {
            if seen_dac && seen_fft { 1 } else { 0 }
        } else {
            let ans = g
                .outputs(node)
                .iter()
                .map(|neighbor| {
                    dfs(
                        g,
                        neighbor,
                        seen_dac || *neighbor == "dac",
                        seen_fft || *neighbor == "fft",
                        dp,
                    )
                })
                .sum::<usize>();
            dp.insert((node, seen_dac, seen_fft), ans);
            ans
        }
    }

    let ans = dfs(&g, "svr", false, false, &mut pathways);
    Some(ans as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = r#"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"#;
        let result = part_two(input);
        assert_eq!(result, Some(2));
    }
}
