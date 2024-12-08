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

type coord struct {
	row, col int
}
type station struct {
	char  byte
	coord coord
}

func isOutside(grid [][]byte, row int, col int) bool {
	return row < 0 || row >= len(grid) || col < 0 || col >= len(grid[row])
}

func Part1(lines []string) string {
	grid := make([][]byte, len(lines))
	for i, line := range lines {
		grid[i] = []byte(line)
	}

	stations := make([]station, 0)
	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[row]); col++ {
			if grid[row][col] != '.' {
				stations = append(stations,
					station{
						grid[row][col],
						coord{
							row, col,
						},
					},
				)
			}
		}
	}

	antiNodes := make([]coord, 0)
	for _, a := range stations {
		for _, b := range stations {
			if a.char == b.char {
				if a != b {
					diff := coord{a.coord.row - b.coord.row, a.coord.col - b.coord.col}
					antinodePos := coord{a.coord.row + diff.row, a.coord.col + diff.col}
					if !isOutside(grid, antinodePos.row, antinodePos.col) {
						grid[antinodePos.row][antinodePos.col] = '#'
						exists := false
						for _, node := range antiNodes {
							if node == antinodePos {
								exists = true
							}
						}
						if !exists {
							antiNodes = append(antiNodes, antinodePos)
						}
					}
				}
			}
		}
	}
	return strconv.Itoa(len(antiNodes))
}

func Part2(lines []string) string {
	grid := make([][]byte, len(lines))
	for i, line := range lines {
		grid[i] = []byte(line)
	}

	stations := make([]station, 0)
	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[row]); col++ {
			if grid[row][col] != '.' {
				stations = append(stations,
					station{
						grid[row][col],
						coord{
							row, col,
						},
					},
				)
			}
		}
	}

	antiNodes := make([]coord, 0)
	for _, a := range stations {
		for _, b := range stations {
			if a.char == b.char {
				if a != b {
					diff := coord{a.coord.row - b.coord.row, a.coord.col - b.coord.col}
					antinodePos := coord{a.coord.row, a.coord.col}
					for !isOutside(grid, antinodePos.row, antinodePos.col) {
						grid[antinodePos.row][antinodePos.col] = '#'
						exists := false
						for _, node := range antiNodes {
							if node == antinodePos {
								exists = true
							}
						}
						if !exists {
							antiNodes = append(antiNodes, antinodePos)
						}
						antinodePos = coord{antinodePos.row + diff.row, antinodePos.col + diff.col}
					}
				}
			}
		}
	}
	return strconv.Itoa(len(antiNodes))
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
