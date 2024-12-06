package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	raw := `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`
	lines := strings.Split(raw, "\n")
	part1Result := Part1(lines)
	part1Solution := "18"
	if part1Result == part1Solution {
		t.Log("Del 1 Riktig:", part1Result, "==", part1Solution)
	} else {
		t.Fatal("Del 1 feil:", part1Result, "!=", part1Solution)
	}
}

func TestPart2(t *testing.T) {
	raw2 := `MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX`
	lines2 := strings.Split(raw2, "\n")
	part2Result := Part2(lines2)
	part2Solution := "9"
	if part2Result == part2Solution {
		t.Log("Del 2 Riktig:", part2Result, "==", part2Solution)
	} else {
		t.Fatal("Del 2 feil:", part2Result, "!=", part2Solution)
	}
}
