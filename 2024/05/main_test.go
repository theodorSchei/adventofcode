package main

import (
	"strings"
	"testing"
)

func TestPart1(t *testing.T) {
	raw := `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`
	lines := strings.Split(raw, "\n")
	part1Result := Part1(lines)
	part1Solution := "143"
	if part1Result == part1Solution {
		t.Log("Del 1 Riktig:", part1Result, "==", part1Solution)
	} else {
		t.Fatal("Del 1 feil:", part1Result, "!=", part1Solution)
	}
}

func TestPart2(t *testing.T) {
	raw2 := `47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47`
	lines2 := strings.Split(raw2, "\n")
	part2Result := Part2(lines2)
	part2Solution := "123"
	if part2Result == part2Solution {
		t.Log("Del 2 Riktig:", part2Result, "==", part2Solution)
	} else {
		t.Fatal("Del 2 feil:", part2Result, "!=", part2Solution)
	}
}
