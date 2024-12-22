package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	input := readInput()
	fmt.Printf("Part 1: %s\n", part1(input))
	fmt.Printf("Part 2: %s\n", part2(input))
}

func readInput() string {
	b, _ := os.ReadFile("../../input/05/input.txt")
	return string(b)
}

func parseStacks(lines []string, scanLine *int) [][]string {
	stacks := make([][]string, 10)
	for *scanLine < 8 {
		line := lines[*scanLine]
		parts := strings.Split(line, " ")
		column := 1
		for i := 0; i < len(parts); i++ {
			if parts[i] == "" {
				// skip column
				column++
				i += 3
			} else {
				// add char to stack
				char := string(parts[i][1])
				stacks[column] = append(stacks[column], char)
				column++
			}
		}
		*scanLine++
	}

	for i := range stacks {
		rev(stacks[i])
	}

	return stacks
}

func rev(arr []string) {
	for i, j := 0, len(arr)-1; i < j; i, j = i+1, j-1 {
		arr[i], arr[j] = arr[j], arr[i]
	}
}

func part1(input string) string {
	lines := strings.Split(input, "\n")
	scanLine := 0

	stacks := parseStacks(lines, &scanLine)
	scanLine += 2

	for scanLine < len(lines) {
		parts := strings.Split(lines[scanLine], " ")
		quant, _ := strconv.Atoi(parts[1])
		from, _ := strconv.Atoi(parts[3])
		to, _ := strconv.Atoi(parts[5])

		for i := 0; i < quant; i++ {
			top := stacks[from][len(stacks[from])-1]
			stacks[to] = append(stacks[to], top)
			stacks[from] = stacks[from][:len(stacks[from])-1]
		}

		scanLine++
	}

	res := ""
	for i := 1; i < len(stacks); i++ {
		res += string(stacks[i][len(stacks[i])-1])
	}
	return res
}

func part2(input string) string {
	lines := strings.Split(input, "\n")
	scanLine := 0

	stacks := parseStacks(lines, &scanLine)
	scanLine += 2

	for scanLine < len(lines) {
		parts := strings.Split(lines[scanLine], " ")
		quant, _ := strconv.Atoi(parts[1])
		from, _ := strconv.Atoi(parts[3])
		to, _ := strconv.Atoi(parts[5])

		move := stacks[from][len(stacks[from])-quant:]
		stacks[to] = append(stacks[to], move...)
		stacks[from] = stacks[from][:len(stacks[from])-quant]

		scanLine++
	}

	res := ""
	for i := 1; i < len(stacks); i++ {
		res += string(stacks[i][len(stacks[i])-1])
	}
	return res
}
