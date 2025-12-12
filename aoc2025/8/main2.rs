use std::io::{self, BufRead};

// Union-Find data structure
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
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
        } else if self.rank[px] > self.rank[py] {
            self.parent[py] = px;
        } else {
            self.parent[py] = px;
            self.rank[px] += 1;
        }
        true
    }

    fn count_circuits(&mut self) -> usize {
        let n = self.parent.len();
        let mut roots = std::collections::HashSet::new();
        for i in 0..n {
            roots.insert(self.find(i));
        }
        roots.len()
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

    // Process connections using Union-Find
    // We need to connect until all junction boxes are in one circuit
    // The last connection that merges two separate circuits into one is our answer
    let mut uf = UnionFind::new(n);
    let mut circuits_remaining = n; // Initially, each junction box is its own circuit

    for (_, i, j) in distances {
        // Only count connections that actually merge two circuits
        if uf.union(i, j) {
            circuits_remaining -= 1;

            // When we reach 1 circuit, we found the last connection
            if circuits_remaining == 1 {
                let x1 = points[i].0 as u64;
                let x2 = points[j].0 as u64;
                return x1 * x2;
            }
        }
    }

    0 // Should not reach here if input is valid
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
        assert_eq!(solve_puzzle(input), 25272);
    }
}
