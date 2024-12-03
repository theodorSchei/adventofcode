package main

import (
	"fmt"
	"os"
	"regexp"
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
	r, _ := regexp.Compile(`mul\((\d{1,3}),(\d{1,3})\)`)
	sum := 0
	for _, line := range lines {
		res := r.FindAllStringSubmatch(line, -1)
		for _, capture := range res {
			a, _ := strconv.Atoi(capture[1])
			b, _ := strconv.Atoi(capture[2])
			sum += a * b
		}
	}
	return strconv.Itoa(sum)
}

func Part2(lines []string) string {
	r, _ := regexp.Compile(`mul\((\d+),(\d+)\)|do\(\)|don't\(\)`)
	sum := 0
	do := true
	for _, line := range lines {
		res := r.FindAllStringSubmatch(line, -1)
		for _, capture := range res {
			println(capture[0])
			if capture[0] == `do()` {
				do = true
				continue
			} else if capture[0] == `don't()` {
				do = false
				continue
			} else {
				if do {
					a, _ := strconv.Atoi(capture[1])
					b, _ := strconv.Atoi(capture[2])
					sum += a * b
				}
			}
		}
	}
	return strconv.Itoa(sum)
}

func main() {
	lines, _ := readLines("./input.txt")
	fmt.Println("Part 1: ", Part1(lines))
	fmt.Println("Part 2: ", Part2(lines))
}
