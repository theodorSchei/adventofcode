package main

import (
	"fmt"
	"os"
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

func reportIsValid(report []int) bool {
	isValid := true
	isIncreasing := report[0] < report[1]
	lastValue := report[0]
	for i := 1; i < len(report); i++ {
		if isIncreasing {
			if lastValue > report[i] {
				isValid = false
				break
			}
			if 0 >= report[i]-lastValue || report[i]-lastValue > 3 {
				isValid = false
				break
			}
		} else {
			if 0 >= lastValue-report[i] || lastValue-report[i] > 3 {
				isValid = false
				break
			}
		}
		lastValue = report[i]
	}
	return isValid
}

func Part1(lines []string) string {
	reports := make([][]int, len(lines))
	for i, line := range lines {
		lineStrings := strings.Split(line, " ")
		reports[i] = make([]int, len(lineStrings))
		for j, stringValue := range lineStrings {
			reports[i][j], _ = strconv.Atoi(stringValue)
		}
	}

	numSafe := 0
	for _, report := range reports {
		isValid := reportIsValid(report)
		if isValid {
			numSafe++
		}
	}

	return strconv.Itoa(numSafe)
}

func Part2(lines []string) string {
	reports := make([][]int, len(lines))
	for i, line := range lines {
		lineStrings := strings.Split(line, " ")
		reports[i] = make([]int, len(lineStrings))
		for j, stringValue := range lineStrings {
			reports[i][j], _ = strconv.Atoi(stringValue)
		}
	}

	numSafe := 0
	for _, report := range reports {
		isValid := reportIsValid(report)
		if !isValid {
			isValid = checkAllVersions(report)
		}
		if isValid {
			numSafe++
		}
	}

	return strconv.Itoa(numSafe)
}

func checkAllVersions(report []int) bool {
	for i := range report {
		modified := make([]int, 0, len(report)-1)
		modified = append(modified, report[:i]...)
		modified = append(modified, report[i+1:]...)

		if reportIsValid(modified) {
			return true
		}
	}
	return false
}

func main() {
	lines, _ := readLines("./input.txt")
	fmt.Println("Part 1: ", Part1(lines))
	fmt.Println("Part 2: ", Part2(lines))
}
