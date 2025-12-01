# Solve Advent of Code Day

Solve a complete Advent of Code day (both parts) automatically.

## Usage

```
/aoc <day-spec>
```

## Parameters

- `day-spec`: Day specification in one of two formats:
  - `<day>` - Day number (1-25) of the current year (2025)
  - `<year>-<day>` - Specific year and day (e.g., `2024-15` for day 15 of 2024)

## Workflow

This command automates the complete AoC solving process for a given day:

### For Part 1:
1. **Fetch puzzle**: Uses `aoc_fetch` to download puzzle1.html and input.md
2. **Extract examples**: Runs `puzzle-extractor` agent to get test data
3. **Solve puzzle**: Delegates to `puzzle-solver` agent to:
   - Write Rust solution (main1.rs)
   - Run tests from examples.json (level 1)
   - Iterate until tests pass
   - Return final answer
4. **Submit answer**: Runs `aoc_submit` to submit the answer to AoC
5. **Save state**: Creates answer1.md (automatically done by aoc_submit)

### For Part 2:
1. **Fetch puzzle**: Uses `aoc_fetch` to download puzzle2.html (now available after part 1 is solved)
2. **Extract examples**: Runs `puzzle-extractor` agent to update examples.json with level 2 test data
3. **Solve puzzle**: Delegates to `puzzle-solver` agent to:
   - Write Rust solution (main2.rs)
   - Run tests from examples.json (level 2)
   - Iterate until tests pass
   - Return final answer
4. **Submit answer**: Runs `aoc_submit` to submit the answer to AoC
5. **Save state**: Creates answer2.md (automatically done by aoc_submit)

## Requirements

- `.env` file with SESSION_TOKEN
- `aoc_fetch` binary compiled in `aoc_fetch/target/release/`
- `aoc_submit` binary compiled in `aoc_submit/target/release/`
- Rust toolchain (rustc)
- `puzzle-extractor` agent configured in `.claude/agents/`
- `puzzle-solver` agent configured in `.claude/agents/`

## Agents Used

### puzzle-solver

Specialized agent for solving Advent of Code puzzles in Rust.

**Responsibilities:**
- Reads puzzle HTML file and understands the problem
- Extracts test cases from examples.json for the target level
- Writes Rust solution code (main1.rs or main2.rs)
- Compiles and runs tests using rustc
- Iterates on the implementation until all tests pass
- Runs solution on actual puzzle input
- Returns the final answer

**Input:**
- Puzzle HTML file path (puzzle1.html or puzzle2.html)
- Examples JSON file path (examples.json)
- Input data file path (input.md)
- Target level (1 or 2)

**Output:**
- Solution file (main1.rs or main2.rs)
- Final answer

### puzzle-extractor

Extracts puzzle input and expected answers from HTML files.

**Responsibilities:**
- Parses HTML puzzle descriptions
- Identifies and extracts example inputs and outputs
- Saves structured test data to examples.json
- Handles both Part 1 and Part 2 puzzles

**Output:**
- examples.json file with test cases

## Command Line Tools Used

### aoc_fetch

Downloads puzzle and input files from Advent of Code.

```
aoc_fetch <day-spec>
```

**Parameters:**
- `day-spec`: Day specification format:
  - `<day>` - Day number of current year (2025)
  - `<year>-<day>` - Specific year and day (e.g., `2024-15`)

**Output:**
- `aocYYYY/DD/puzzle1.html` - Part 1 puzzle description
- `aocYYYY/DD/puzzle2.html` - Part 2 puzzle description (after Part 1 is solved)
- `aocYYYY/DD/input.md` - Puzzle input data

**Logic:**
- Checks for `aocYYYY/DD/answer1.md` to determine if Part 1 is solved
- If answer1.md exists, fetches puzzle2.html; otherwise fetches puzzle1.html
- Only downloads input.md on the first run (Part 1)
- Reads SESSION_TOKEN from `.env` file
- Auto-detects current year if not specified in day-spec

### aoc_submit

Submits answers to Advent of Code.

```
aoc_submit <day-spec> <level> <answer>
```

**Parameters:**
- `day-spec`: Day specification format:
  - `<day>` - Day number of current year (2025)
  - `<year>-<day>` - Specific year and day (e.g., `2024-15`)
- `level`: The puzzle level (1 or 2)
- `answer`: The answer to submit

**Output:**
- Posts answer to `https://adventofcode.com/YYYY/day/DD/answer`
- Creates `aocYYYY/DD/answerN.md` if the answer is correct
- Displays feedback (correct/wrong/already solved/rate limited)

**Session:**
- Reads SESSION_TOKEN from `.env` file
- Auto-detects current year if not specified in day-spec

## Output Structure

After running `/aoc <day-spec>`, the following files are created in `aocYYYY/DD/`:
- `puzzle1.html` - Part 1 description
- `puzzle2.html` - Part 2 description
- `input.md` - Puzzle input data
- `examples.json` - Test cases for both parts
- `main1.rs` - Part 1 solution
- `main2.rs` - Part 2 solution
- `answer1.md` - Part 1 answer
- `answer2.md` - Part 2 answer

## Examples

```bash
# Solve day 5 of 2025 (current year)
/aoc 5

# Solve day 15 of 2024
/aoc 2024-15

# Directly use command line tools
./aoc_fetch 10               # Fetch day 10 of 2025
./aoc_fetch 2023-20          # Fetch day 20 of 2023
./aoc_submit 10 1 42         # Submit answer 42 for day 10 level 1 of 2025
./aoc_submit 2023-20 2 9999  # Submit answer 9999 for day 20 of 2023 level 2
```
