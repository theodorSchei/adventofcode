package main

import (
	"strings"
	"testing"
)

func TestSolution(t *testing.T) {
	raw := `3   4
4   3
2   5
1   3
3   9
3   3`
	lines := strings.Split(raw, "\n")
	part1Result := Part1(lines)
	part1Solution := "11"
	if part1Result != part1Solution {
		t.Fatal("Del 1 feil: ", part1Result, " != ", part1Solution)
	}
	part2Result := Part2(lines)
	part2Solution := "31"

	if part2Result != part2Solution {
		t.Fatal("Del 2 feil: ", part2Result, " != ", part2Solution)
	}
}
