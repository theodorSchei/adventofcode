package main

import (
	"fmt"
	"os"
	"runtime"
	"strconv"
	"strings"
	"sync"
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
	North   = DirectionOffset{-1, 0}
	South   = DirectionOffset{1, 0}
	East    = DirectionOffset{0, 1}
	West    = DirectionOffset{0, -1}
	nextDir = map[DirectionOffset]DirectionOffset{
		North: East,
		East:  South,
		South: West,
		West:  North,
	}
)

func getValue(grid [][]byte, row int, col int) byte {
	if row < 0 || row >= len(grid) || col < 0 || col >= len(grid[row]) {
		return '0'
	}
	return grid[row][col]
}

func isOnEdge(grid [][]byte, row int, col int) bool {
	return row == 0 || row == len(grid)-1 || col == 0 || col == len(grid[row])-1
}

func printGrid(grid [][]byte) {
	for _, row := range grid {
		for _, cell := range row {
			fmt.Printf("%c ", cell)
		}
		fmt.Println()
	}
	fmt.Println()
}

func Part1(lines []string) string {
	grid := make([][]byte, len(lines))
	for i, line := range lines {
		grid[i] = []byte(line)
	}

	curRow, curCol := 0, 0
	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[row]); col++ {
			if grid[row][col] == '^' {
				curRow = row
				curCol = col
			}
		}
	}
	curDir := North

	for !isOnEdge(grid, curRow, curCol) {
		nextStep := getValue(grid, curRow+curDir.rowOffset, curCol+curDir.colOffset)
		switch nextStep {
		case '.', 'X':
			grid[curRow][curCol] = 'X' // Visited
			curRow = curRow + curDir.rowOffset
			curCol = curCol + curDir.colOffset
		case '#':
			curDir = nextDir[curDir]
		}
	}

	visited := 1
	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[row]); col++ {
			if grid[row][col] == 'X' {
				visited++
			}
		}
	}

	return strconv.Itoa(visited)
}

func isSolvable(grid [][]byte, curRow int, curCol int, curDir DirectionOffset) bool {
	maxNumberOfSteps := len(grid) * len(grid[0])

	for n := 0; !isOnEdge(grid, curRow, curCol); n++ {
		nextStep := getValue(grid, curRow+curDir.rowOffset, curCol+curDir.colOffset)
		switch nextStep {
		case '.', 'X', '0':
			grid[curRow][curCol] = 'X' // Visited
			curRow = curRow + curDir.rowOffset
			curCol = curCol + curDir.colOffset
		case '#':
			curDir = nextDir[curDir]
		}
		if n > maxNumberOfSteps {
			return false
		}
	}
	return true
}

func Part2(lines []string) string {
	grid := make([][]byte, len(lines))
	for i, line := range lines {
		grid[i] = []byte(line)
	}

	// Find starting position
	curRow, curCol := 0, 0
	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[row]); col++ {
			if grid[row][col] == '^' {
				curRow = row
				curCol = col
			}
		}
	}
	curDir := North
	spawnRow, spawnCol := curRow, curCol

	// First solve to mark path
	for !isOnEdge(grid, curRow, curCol) {
		nextStep := getValue(grid, curRow+curDir.rowOffset, curCol+curDir.colOffset)
		switch nextStep {
		case '.', 'X':
			grid[curRow][curCol] = 'X'
			curRow = curRow + curDir.rowOffset
			curCol = curCol + curDir.colOffset
		case '#':
			curDir = nextDir[curDir]
		}
	}
	grid[curRow][curCol] = 'X'

	// Collect visited positions
	var visitedPositions [][2]int
	for row := 0; row < len(grid); row++ {
		for col := 0; col < len(grid[row]); col++ {
			if grid[row][col] == 'X' {
				visitedPositions = append(visitedPositions, [2]int{row, col})
			}
		}
	}

	// Parallel processing
	numWorkers := runtime.NumCPU()
	results := make(chan int, numWorkers)
	var wg sync.WaitGroup

	positionsPerWorker := len(visitedPositions) / numWorkers
	if len(visitedPositions)%numWorkers != 0 {
		positionsPerWorker++
	}

	for worker := 0; worker < numWorkers; worker++ {
		wg.Add(1)
		go func(workerID int) {
			defer wg.Done()
			localUnsolvable := 0
			start := workerID * positionsPerWorker
			end := min((workerID+1)*positionsPerWorker, len(visitedPositions))

			// Local grid for this worker
			workerGrid := make([][]byte, len(grid))
			for i := range grid {
				workerGrid[i] = make([]byte, len(grid[i]))
			}

			for i := start; i < end; i++ {
				pos := visitedPositions[i]
				row, col := pos[0], pos[1]

				// Copy grid for this test
				for i := range grid {
					copy(workerGrid[i], grid[i])
				}
				workerGrid[row][col] = '#'

				if !isSolvable(workerGrid, spawnRow, spawnCol, curDir) {
					localUnsolvable++
				}
			}
			results <- localUnsolvable
		}(worker)
	}

	// Close results channel after all workers finish
	go func() {
		wg.Wait()
		close(results)
	}()

	// Sum up results
	numUnsolvable := 0
	for result := range results {
		numUnsolvable += result
	}

	return strconv.Itoa(numUnsolvable)
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
