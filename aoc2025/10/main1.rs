use std::io::{self, BufRead};

fn parse_line(line: &str) -> (u64, Vec<u64>) {
    // Parse indicator light diagram [.##.]
    let bracket_start = line.find('[').unwrap();
    let bracket_end = line.find(']').unwrap();
    let diagram = &line[bracket_start + 1..bracket_end];

    let mut target: u64 = 0;
    for (i, c) in diagram.chars().enumerate() {
        if c == '#' {
            target |= 1 << i;
        }
    }

    // Parse button wiring schematics (0,2,3,4) etc.
    let mut buttons: Vec<u64> = Vec::new();
    let rest = &line[bracket_end + 1..];

    // Find all parentheses groups
    let mut i = 0;
    let chars: Vec<char> = rest.chars().collect();
    while i < chars.len() {
        if chars[i] == '(' {
            let start = i + 1;
            while i < chars.len() && chars[i] != ')' {
                i += 1;
            }
            let end = i;
            let content: String = chars[start..end].iter().collect();

            let mut button_mask: u64 = 0;
            for num_str in content.split(',') {
                let num_str = num_str.trim();
                if !num_str.is_empty() {
                    if let Ok(num) = num_str.parse::<usize>() {
                        button_mask |= 1 << num;
                    }
                }
            }
            buttons.push(button_mask);
        }
        i += 1;
    }

    (target, buttons)
}

fn solve_machine(target: u64, buttons: &[u64]) -> u32 {
    // We need to find the minimum number of buttons to press
    // such that their XOR equals target.
    // Since pressing a button twice cancels out, each button is pressed 0 or 1 times.
    // This is a minimum weight solution to a system of linear equations over GF(2).

    // We use BFS/dynamic programming with meet-in-the-middle for larger button sets,
    // but for small sets we can use simple BFS or brute force.

    let n = buttons.len();

    if n <= 20 {
        // Brute force with pruning - try all subsets in order of size
        for num_presses in 0..=n {
            if find_subset_with_size(target, buttons, num_presses as u32) {
                return num_presses as u32;
            }
        }
    } else {
        // Meet in the middle approach
        return meet_in_middle(target, buttons);
    }

    u32::MAX // No solution found
}

fn find_subset_with_size(target: u64, buttons: &[u64], size: u32) -> bool {
    let n = buttons.len();
    if size == 0 {
        return target == 0;
    }

    // Generate all combinations of 'size' buttons
    fn combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
        let mut result = Vec::new();
        let mut current = Vec::new();
        fn generate(start: usize, n: usize, k: usize, current: &mut Vec<usize>, result: &mut Vec<Vec<usize>>) {
            if current.len() == k {
                result.push(current.clone());
                return;
            }
            if start >= n || n - start < k - current.len() {
                return;
            }
            current.push(start);
            generate(start + 1, n, k, current, result);
            current.pop();
            generate(start + 1, n, k, current, result);
        }
        generate(0, n, k, &mut current, &mut result);
        result
    }

    for combo in combinations(n, size as usize) {
        let mut xor_val: u64 = 0;
        for &idx in &combo {
            xor_val ^= buttons[idx];
        }
        if xor_val == target {
            return true;
        }
    }
    false
}

fn meet_in_middle(target: u64, buttons: &[u64]) -> u32 {
    use std::collections::HashMap;

    let n = buttons.len();
    let half = n / 2;

    // First half: compute all possible XOR values and minimum presses needed
    let mut first_half: HashMap<u64, u32> = HashMap::new();
    for mask in 0u64..(1 << half) {
        let mut xor_val: u64 = 0;
        let mut count = 0u32;
        for i in 0..half {
            if mask & (1 << i) != 0 {
                xor_val ^= buttons[i];
                count += 1;
            }
        }
        let entry = first_half.entry(xor_val).or_insert(u32::MAX);
        *entry = (*entry).min(count);
    }

    // Second half: for each subset, check what we need from first half
    let mut min_presses = u32::MAX;
    let second_len = n - half;
    for mask in 0u64..(1 << second_len) {
        let mut xor_val: u64 = 0;
        let mut count = 0u32;
        for i in 0..second_len {
            if mask & (1 << i) != 0 {
                xor_val ^= buttons[half + i];
                count += 1;
            }
        }
        // We need first_half XOR to be (target XOR xor_val)
        let needed = target ^ xor_val;
        if let Some(&first_count) = first_half.get(&needed) {
            min_presses = min_presses.min(count + first_count);
        }
    }

    min_presses
}

fn solve_puzzle(input: &str) -> u64 {
    let mut total: u64 = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (target, buttons) = parse_line(line);
        let min_presses = solve_machine(target, &buttons);
        total += min_presses as u64;
    }

    total
}

fn main() {
    let stdin = io::stdin();
    let input: String = stdin.lock().lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<_>>()
        .join("\n");

    let result = solve_puzzle(&input);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
        assert_eq!(solve_puzzle(input), 2);
    }

    #[test]
    fn test_example2() {
        let input = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
        assert_eq!(solve_puzzle(input), 3);
    }

    #[test]
    fn test_example3() {
        let input = "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve_puzzle(input), 2);
    }

    #[test]
    fn test_all_examples() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(solve_puzzle(input), 7);
    }
}
