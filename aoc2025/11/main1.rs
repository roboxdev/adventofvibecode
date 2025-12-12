use std::collections::HashMap;
use std::io::{self, BufRead};

fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    // Base case: we reached the target
    if current == target {
        return 1;
    }

    // Check if we've already computed paths from this node
    if let Some(&count) = memo.get(current) {
        return count;
    }

    // Get children of current node
    let count = if let Some(children) = graph.get(current) {
        children
            .iter()
            .map(|child| count_paths(graph, child, target, memo))
            .sum()
    } else {
        // No outgoing edges from this node, dead end
        0
    };

    memo.insert(current.to_string(), count);
    count
}

fn solve_puzzle(input: &str) -> u64 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse line: "node: child1 child2 child3"
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let node = parts[0].trim().to_string();
        let children: Vec<String> = parts[1]
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        graph.insert(node, children);
    }

    let mut memo: HashMap<String, u64> = HashMap::new();
    count_paths(&graph, "you", "out", &mut memo)
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>().join("\n");
    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(solve_puzzle(input), 5);
    }
}
