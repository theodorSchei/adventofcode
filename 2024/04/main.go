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

type DirectionOffset struct {
	rowOffset, colOffset int
}

var (
	North         = DirectionOffset{-1, 0}
	South         = DirectionOffset{1, 0}
	East          = DirectionOffset{0, 1}
	West          = DirectionOffset{0, -1}
	NorthEast     = DirectionOffset{-1, 1}
	NorthWest     = DirectionOffset{-1, -1}
	SouthEast     = DirectionOffset{1, 1}
	SouthWest     = DirectionOffset{1, -1}
	AllDirections = []DirectionOffset{North, South, East, West, NorthEast, NorthWest, SouthEast, SouthWest}
)

func getValue(grid [][]byte, row int, col int) (byte, bool) {
	if row < 0 || row >= len(grid) || col < 0 || col >= len(grid[row]) {
		return 0, false
	}
	return grid[row][col], true
}

func Part1(lines []string) string {
	grid := make([][]byte, len(lines))
	for i, line := range lines {
		grid[i] = []byte(line)
	}

	word := "XMAS"
	matches := 0
	for row := range grid {
		for col := range grid[row] {
			for _, dir := range AllDirections {
				isValid := true
				for i := 0; i < len(word); i++ {
					val, ok := getValue(grid, row+(dir.rowOffset*i), col+(dir.colOffset*i))
					if !ok || val != word[i] {
						isValid = false
						break
					}
				}
				if isValid {
					matches++
				}
			}
		}
	}

	return strconv.Itoa(matches)
}

func hasDiagonalMAS(grid [][]byte, row, col int) bool {
	topLeft, hasTopLeft := getValue(grid, row+NorthEast.rowOffset, col+NorthEast.colOffset)
	topRight, hasTopRight := getValue(grid, row+NorthWest.rowOffset, col+NorthWest.colOffset)
	bottomLeft, hasBottomLeft := getValue(grid, row+SouthEast.rowOffset, col+SouthEast.colOffset)
	bottomRight, hasBottomRight := getValue(grid, row+SouthWest.rowOffset, col+SouthWest.colOffset)

	if hasTopLeft && hasTopRight && hasBottomLeft && hasBottomRight {
		if (topLeft == 'M' && topRight == 'M' && bottomLeft == 'S' && bottomRight == 'S') ||
			(topLeft == 'S' && topRight == 'S' && bottomLeft == 'M' && bottomRight == 'M') ||
			(topLeft == 'M' && bottomLeft == 'M' && topRight == 'S' && bottomRight == 'S') ||
			(topLeft == 'S' && bottomLeft == 'S' && topRight == 'M' && bottomRight == 'M') {
			return true
		}
	}
	return false
}

func Part2(lines []string) string {
	grid := make([][]byte, len(lines))
	for i, line := range lines {
		grid[i] = []byte(line)
	}

	matches := 0
	for row := range grid {
		for col := range grid[row] {
			if grid[row][col] == 'A' {
				if hasDiagonalMAS(grid, row, col) {
					matches++
				}
			}
		}
	}

	return strconv.Itoa(matches)
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
