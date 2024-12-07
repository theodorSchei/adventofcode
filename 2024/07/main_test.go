package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	raw := `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`
	lines := strings.Split(raw, "\n")
	part1Result := Part1(lines)
	part1Solution := "3749"
	if part1Result == part1Solution {
		t.Log("Del 1 Riktig:", part1Result, "==", part1Solution)
	} else {
		t.Fatal("Del 1 feil:", part1Result, "!=", part1Solution)
	}
}

func TestPart2(t *testing.T) {
	raw2 := `190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20`
	lines2 := strings.Split(raw2, "\n")
	part2Result := Part2(lines2)
	part2Solution := "11387"
	if part2Result == part2Solution {
		t.Log("Del 2 Riktig:", part2Result, "==", part2Solution)
	} else {
		t.Fatal("Del 2 feil:", part2Result, "!=", part2Solution)
	}
}
