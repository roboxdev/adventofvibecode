use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(|l| l.ok()).collect();
    let result = solve_puzzle(&lines);
    println!("{}", result);
}

fn solve_puzzle(lines: &[String]) -> u64 {
    // Parse the graph: device -> list of outputs
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let device = parts[0].trim().to_string();
        let outputs: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        graph.insert(device, outputs);
    }

    // Memoization: for each node, store (paths_with_neither, paths_with_dac_only, paths_with_fft_only, paths_with_both)
    // These represent paths from that node to "out"
    let mut memo: HashMap<String, (u64, u64, u64, u64)> = HashMap::new();

    // Use topological sort (DFS post-order) to compute the values
    let mut visited: HashMap<String, bool> = HashMap::new(); // false = in progress, true = done

    fn compute(
        node: &str,
        graph: &HashMap<String, Vec<String>>,
        memo: &mut HashMap<String, (u64, u64, u64, u64)>,
        visited: &mut HashMap<String, bool>,
    ) -> (u64, u64, u64, u64) {
        // Check if already computed
        if let Some(&result) = memo.get(node) {
            return result;
        }

        // Check for cycle (shouldn't happen in a DAG)
        if visited.get(node) == Some(&false) {
            // Cycle detected, return zeros
            return (0, 0, 0, 0);
        }

        // Mark as in progress
        visited.insert(node.to_string(), false);

        // Base case: "out" node
        if node == "out" {
            let result = (1, 0, 0, 0); // One path with neither dac nor fft
            memo.insert(node.to_string(), result);
            visited.insert(node.to_string(), true);
            return result;
        }

        // If no outputs, this is a dead end (not "out")
        if !graph.contains_key(node) {
            let result = (0, 0, 0, 0);
            memo.insert(node.to_string(), result);
            visited.insert(node.to_string(), true);
            return result;
        }

        // Sum up paths from all children
        let mut total_neither = 0u64;
        let mut total_dac_only = 0u64;
        let mut total_fft_only = 0u64;
        let mut total_both = 0u64;

        for next in &graph[node] {
            let (neither, dac_only, fft_only, both) = compute(next, graph, memo, visited);
            total_neither += neither;
            total_dac_only += dac_only;
            total_fft_only += fft_only;
            total_both += both;
        }

        // Now adjust based on whether this node is dac or fft
        let result = if node == "dac" {
            // All paths through this node now have dac visited
            // neither -> dac_only
            // fft_only -> both
            // dac_only stays dac_only
            // both stays both
            (0, total_neither + total_dac_only, 0, total_fft_only + total_both)
        } else if node == "fft" {
            // All paths through this node now have fft visited
            // neither -> fft_only
            // dac_only -> both
            // fft_only stays fft_only
            // both stays both
            (0, 0, total_neither + total_fft_only, total_dac_only + total_both)
        } else {
            // Regular node, just pass through
            (total_neither, total_dac_only, total_fft_only, total_both)
        };

        memo.insert(node.to_string(), result);
        visited.insert(node.to_string(), true);
        result
    }

    let (_, _, _, both) = compute("svr", &graph, &mut memo, &mut visited);
    both
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level2_example() {
        let input = "svr: aaa bbb
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
hhh: out";
        let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
        assert_eq!(solve_puzzle(&lines), 2);
    }
}
