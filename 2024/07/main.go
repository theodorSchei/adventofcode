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

func evaluateExpression(numbers []int, operators []byte) int {
	result := numbers[0]
	for i := 0; i < len(operators); i++ {
		switch operators[i] {
		case '+':
			result += numbers[i+1]
		case '*':
			result *= numbers[i+1]
		case '|':
			resultStr := strconv.Itoa(result)
			nextStr := strconv.Itoa(numbers[i+1])
			result, _ = strconv.Atoi(resultStr + nextStr)
		}
	}
	return result
}

func generateOperatorCombinations(allowedOperators []byte, length int) [][]byte {
	if length == 0 {
		return [][]byte{{}}
	}

	var combinations [][]byte
	for _, op := range allowedOperators {
		subCombinations := generateOperatorCombinations(allowedOperators, length-1)
		for _, subComb := range subCombinations {
			combination := append([]byte{op}, subComb...)
			combinations = append(combinations, combination)
		}
	}
	return combinations
}

func processLine(line string, allowedOperators []byte) int {
	parts := strings.Split(line, ": ")
	targetResult, _ := strconv.Atoi(parts[0])

	numStrs := strings.Fields(parts[1])
	numbers := make([]int, len(numStrs))
	for i, numStr := range numStrs {
		numbers[i], _ = strconv.Atoi(numStr)
	}

	operatorCombinations := generateOperatorCombinations(allowedOperators, len(numbers)-1)

	for _, operators := range operatorCombinations {
		if evaluateExpression(numbers, operators) == targetResult {
			return targetResult
		}
	}
	return 0
}

func Part1(lines []string) string {
	sum := 0
	allowedOperators := []byte{'+', '*'}

	for _, line := range lines {
		sum += processLine(line, allowedOperators)
	}
	return strconv.Itoa(sum)
}

func Part2(lines []string) string {
	allowedOperators := []byte{'+', '*', '|'}
	numWorkers := runtime.NumCPU() // Use number of available CPU cores

	// Create work channels
	jobs := make(chan string, len(lines))
	results := make(chan int, len(lines))

	// Create worker pool
	var wg sync.WaitGroup
	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for line := range jobs {
				result := processLine(line, allowedOperators)
				results <- result
			}
		}()
	}

	// Send jobs
	for _, line := range lines {
		jobs <- line
	}
	close(jobs)

	// Wait for all workers in separate goroutine
	go func() {
		wg.Wait()
		close(results)
	}()

	// Collect results
	sum := 0
	for result := range results {
		sum += result
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
