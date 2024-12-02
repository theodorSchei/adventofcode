package main

import (
	"strings"
	"testing"
)

func TestSolution(t *testing.T) {
	raw := `7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9`
	lines := strings.Split(raw, "\n")
	part1Result := Part1(lines)
	part1Solution := "2"
	if part1Result != part1Solution {
		t.Fatal("Del 1 feil: ", part1Result, " != ", part1Solution)
	}
	part2Result := Part2(lines)
	part2Solution := "4"

	if part2Result != part2Solution {
		t.Fatal("Del 2 feil: ", part2Result, " != ", part2Solution)
	}
}
