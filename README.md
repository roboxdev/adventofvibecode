# adventofvibecode

Automated Advent of Code solver in Rust with Claude Code integration.

## Quick Start

```bash
claude --dangerously-skip-permissions "/aoc $(date +%Y)-$(date +%-d)"
```


## Usage

```claude
# Solve day 5 of 2025
/aoc 5

# Solve day 15 of 2024
/aoc 2024-15
```

## Setup

1. Create `.env` with your AoC session token:
```
SESSION_TOKEN=your_session_token_here
```

2. Build tools:
```bash
cargo build --release -p aoc_fetch
cargo build --release -p aoc_submit
```

## Components

- **aoc_fetch**: Downloads puzzle HTML and input files
- **aoc_submit**: Submits answers to Advent of Code
- **puzzle-solver**: Claude agent that solves puzzles in Rust
- **puzzle-extractor**: Claude agent that extracts test cases from HTML

## Output

Solutions saved to `aocYYYY/DD/`:
- `puzzle1.html`, `puzzle2.html` - Puzzle descriptions
- `input.md` - Puzzle input
- `examples.json` - Test cases
- `main1.rs`, `main2.rs` - Solutions
- `answer1.md`, `answer2.md` - Answers

## Command Line Tools

```bash
# Fetch puzzle
./aoc_fetch 5           # Day 5 of 2025
./aoc_fetch 2024-15     # Day 15 of 2024

# Submit answer
./aoc_submit 5 1 12345          # Day 5 level 1 of 2025
./aoc_submit 2024-15 2 67890    # Day 15 level 2 of 2024
```

## Requirements

- Rust toolchain
- Claude Code with agents support
- Advent of Code session token
