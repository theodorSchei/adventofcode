package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	raw := `xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))`
	lines := strings.Split(raw, "\n")
	part1Result := Part1(lines)
	part1Solution := "161"
	if part1Result == part1Solution {
		t.Log("Del 1 Riktig:", part1Result, "==", part1Solution)
	} else {
		t.Fatal("Del 1 feil:", part1Result, "!=", part1Solution)
	}
}

func TestPart2(t *testing.T) {
	raw2 := `xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))`
	lines2 := strings.Split(raw2, "\n")
	part2Result := Part2(lines2)
	part2Solution := "48"
	if part2Result == part2Solution {
		t.Log("Del 2 Riktig:", part2Result, "==", part2Solution)
	} else {
		t.Fatal("Del 2 feil:", part2Result, "!=", part2Solution)
	}
}
