package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
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
	emptyLineIndex := 0
	for i, line := range lines {
		if line == "" {
			emptyLineIndex = i
		}
	}

	orderings := make(map[int]map[int]bool)

	for _, order := range lines[:emptyLineIndex] {
		split := strings.Split(order, "|")
		before, _ := strconv.Atoi(split[0])
		after, _ := strconv.Atoi(split[1])
		if orderings[before] == nil {
			orderings[before] = make(map[int]bool)
		}
		orderings[before][after] = true
	}

	updates := make([][]int, len(lines[emptyLineIndex+1:]))
	for i, line := range lines[emptyLineIndex+1:] {
		split := strings.Split(line, ",")
		update := make([]int, len(split))
		for j, char := range split {
			num, _ := strconv.Atoi(char)
			update[j] = num
		}
		updates[i] = update
	}

	sum := 0
	for _, update := range updates {
		isValid := true
		for i := 0; i < len(update); i++ {
			current := update[i]
			for j := 0; j < i; j++ {
				if orderings[current][update[j]] {
					isValid = false
				}
			}
		}
		if isValid {
			sum += update[len(update)/2]
		}
	}

	return strconv.Itoa(sum)
}

func isValid(update []int, orderings map[int]map[int]bool) bool {
	for i := 0; i < len(update); i++ {
		current := update[i]
		for j := 0; j < i; j++ {
			if orderings[current][update[j]] {
				return false
			}
		}
	}
	return true
}

func Part2(lines []string) string {
	emptyLineIndex := 0
	for i, line := range lines {
		if line == "" {
			emptyLineIndex = i
		}
	}

	orderings := make(map[int]map[int]bool)

	for _, order := range lines[:emptyLineIndex] {
		split := strings.Split(order, "|")
		before, _ := strconv.Atoi(split[0])
		after, _ := strconv.Atoi(split[1])
		if orderings[before] == nil {
			orderings[before] = make(map[int]bool)
		}
		orderings[before][after] = true
	}

	updates := make([][]int, len(lines[emptyLineIndex+1:]))
	for i, line := range lines[emptyLineIndex+1:] {
		split := strings.Split(line, ",")
		update := make([]int, len(split))
		for j, char := range split {
			num, _ := strconv.Atoi(char)
			update[j] = num
		}
		updates[i] = update
	}

	sum := 0
	for _, update := range updates {
		valid := isValid(update, orderings)
		if !valid {
			for {
				for i := 0; i < len(update); i++ {
					current := update[i]
					for j := 0; j < i; j++ {
						if orderings[current][update[j]] {
							// update[i] is wrong, move one left
							temp := update[i]
							update[i] = update[i-1]
							update[i-1] = temp
						}
					}
				}
				if isValid(update, orderings) {
					break
				}
			}
			sum += update[len(update)/2]
		}
	}

	return strconv.Itoa(sum)
}

func main() {
	start := time.Now()
	lines, _ := readLines("./input.txt")
	readTime := time.Since(start)

	part1Start := time.Now()
	part1 := Part1(lines)
	part1Time := time.Since(part1Start)

	part2Start := time.Now()
	part2 := Part2(lines)
	part2Time := time.Since(part2Start)

	totalTime := time.Since(start)

	fmt.Printf("Read %d lines in %s\n", len(lines), readTime)
	fmt.Printf("Part 1: %-16s (%s)\n", part1, part1Time)
	fmt.Printf("Part 2: %-16s (%s)\n", part2, part2Time)
	fmt.Printf("Total: %s\n", totalTime)
}
