# Run today's Advent of Code puzzle
aoc:
	claude --dangerously-skip-permissions "/aoc $$(date +%Y)-$$(date +%-d)"
