package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func readLines(filePath string) ([]string, error) {
	data, err := os.ReadFile(filePath)
	if err != nil {
		return nil, fmt.Errorf("error reading file: %v", err)
	}
	content := strings.TrimSpace(string(data))
	lines := strings.Split(string(content), "\n")

	return lines, nil
}

func Part1(lines []string) string {
	a := make([]int, len(lines))
	b := make([]int, len(lines))
	for i := 0; i < len(lines); i++ {
		line := strings.Split(lines[i], "   ")
		if len(line) != 2 {
			break
		}
		a[i], _ = strconv.Atoi(line[0])
		b[i], _ = strconv.Atoi(line[1])
	}
	sort.Slice(a, func(i, j int) bool {
		return a[i] < a[j]
	})
	sort.Slice(b, func(i, j int) bool {
		return b[i] < b[j]
	})
	sum := 0
	for i := 0; i < len(lines); i++ {
		if a[i] > b[i] {
			sum += a[i] - b[i]
		} else {
			sum += b[i] - a[i]
		}
	}
	return strconv.Itoa(sum)
}

func Part2(lines []string) string {
	a := make([]int, len(lines))
	bFrequencies := map[int]int{}
	for i := 0; i < len(lines); i++ {
		line := strings.Split(lines[i], "   ")
		if len(line) != 2 {
			break
		}
		a[i], _ = strconv.Atoi(line[0])
		b, _ := strconv.Atoi(line[1])
		bFrequencies[b]++
	}

	sum := 0
	for i := 0; i < len(a); i++ {
		sum += a[i] * bFrequencies[a[i]]
	}
	return strconv.Itoa(sum)
}

func main() {
	lines, _ := readLines("./input.txt")
	fmt.Println("Part 1: ", Part1(lines))
	fmt.Println("Part 2: ", Part2(lines))
}
