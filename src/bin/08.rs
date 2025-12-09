use std::collections::HashSet;

advent_of_code::solution!(8);

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let px = self.find(x);
        let py = self.find(y);

        if px != py {
            if self.size[px] < self.size[py] {
                self.parent[px] = py;
                self.size[py] += self.size[px];
            } else {
                self.parent[py] = px;
                self.size[px] += self.size[py];
            }
        }
    }

    fn circuit_sizes(&mut self) -> Vec<usize> {
        let mut roots = HashSet::new();

        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }

        roots.iter().map(|&r| self.size[r]).collect()
    }
}

struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn from_line(line: &str) -> Self {
        let mut nums = line.split(',').map(|n| n.parse().unwrap());
        Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
            z: nums.next().unwrap(),
        }
    }

    fn distance(&self, other: &JunctionBox) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input.lines().map(JunctionBox::from_line).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let boxes = parse_input(input);
    let n = boxes.len();

    let mut edges: Vec<(usize, usize, u64)> = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            edges.push((i, j, boxes[i].distance(&boxes[j])));
        }
    }

    edges.sort_unstable_by_key(|e| e.2);

    // to work with example and puzzle input
    let num_connections = if n <= 20 { 10 } else { 1000 };

    let mut union_find = UnionFind::new(n);

    for &(i, j, _) in edges.iter().take(num_connections) {
        union_find.union(i, j);
    }

    let mut sizes = union_find.circuit_sizes();
    sizes.sort_unstable();

    Some(sizes.iter().rev().take(3).map(|&s| s as u64).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes = parse_input(input);
    let n = boxes.len();

    let mut edges: Vec<(usize, usize, u64)> = Vec::new();

    for i in 0..n {
        for j in i + 1..n {
            edges.push((i, j, boxes[i].distance(&boxes[j])));
        }
    }

    edges.sort_unstable_by_key(|e| e.2);

    let mut union_find = UnionFind::new(n);

    for &(i, j, _) in edges.iter() {
        if union_find.find(i) != union_find.find(j) {
            union_find.union(i, j);

            if union_find.circuit_sizes().len() == 1 {
                return Some(boxes[i].x * boxes[j].x);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
