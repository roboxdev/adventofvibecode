use std::io::{self, BufRead};
use std::collections::HashMap;

// Union-Find data structure
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let px = self.find(x);
        let py = self.find(y);

        if px == py {
            return false; // Already in the same circuit
        }

        // Union by rank
        if self.rank[px] < self.rank[py] {
            self.parent[px] = py;
            self.size[py] += self.size[px];
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
            self.size[px] += self.size[py];
        } else {
            self.parent[py] = px;
            self.size[px] += self.size[py];
            self.rank[px] += 1;
        }
        true
    }

    fn get_circuit_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes: HashMap<usize, usize> = HashMap::new();

        for i in 0..n {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }

        sizes.values().cloned().collect()
    }
}

fn solve_puzzle(input: &str) -> u64 {
    // Parse input
    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<i64> = line.split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            (parts[0], parts[1], parts[2])
        })
        .collect();

    let n = points.len();

    // Calculate all pairwise distances (squared to avoid sqrt)
    let mut distances: Vec<(u64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = (points[i].0 - points[j].0).abs() as u64;
            let dy = (points[i].1 - points[j].1).abs() as u64;
            let dz = (points[i].2 - points[j].2).abs() as u64;
            let dist_sq = dx * dx + dy * dy + dz * dz;
            distances.push((dist_sq, i, j));
        }
    }

    // Sort by distance
    distances.sort_by_key(|&(d, _, _)| d);

    // Determine the number of connections to make
    // For the example with 20 boxes, we make 10 connections
    // For the real input with 1000 boxes, we make 1000 connections
    let num_connections = if n == 20 { 10 } else { 1000 };

    // Process connections using Union-Find
    let mut uf = UnionFind::new(n);
    let mut connections_made = 0;
    let mut idx = 0;

    while connections_made < num_connections && idx < distances.len() {
        let (_, i, j) = distances[idx];
        // We count even if already connected (as per puzzle description)
        // Actually, re-reading: "the Elves would like to focus on connecting pairs of junction boxes"
        // So we attempt to connect the 1000 closest pairs regardless of whether they're already connected
        uf.union(i, j);
        connections_made += 1;
        idx += 1;
    }

    // Get circuit sizes
    let mut sizes = uf.get_circuit_sizes();
    sizes.sort_by(|a, b| b.cmp(a)); // Sort descending

    // Multiply the three largest
    let result = if sizes.len() >= 3 {
        (sizes[0] as u64) * (sizes[1] as u64) * (sizes[2] as u64)
    } else if sizes.len() == 2 {
        (sizes[0] as u64) * (sizes[1] as u64)
    } else if sizes.len() == 1 {
        sizes[0] as u64
    } else {
        0
    };

    result
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(solve_puzzle(input), 40);
    }
}
