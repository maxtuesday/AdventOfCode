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
	b, _ := os.ReadFile("../../input/04/input.txt")
	return string(b)
}

func parseRanges(line string) ([]int, []int) {
	parts := strings.Split(line, ",")
	left := strings.Split(parts[0], "-")
	a, _ := strconv.Atoi(left[0])
	b, _ := strconv.Atoi(left[1])

	right := strings.Split(parts[1], "-")
	c, _ := strconv.Atoi(right[0])
	d, _ := strconv.Atoi(right[1])

	return []int{a, b}, []int{c, d}
}

func part1(input string) string {
	res := 0
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		r1, r2 := parseRanges(line)

		if r1[0] <= r2[0] && r2[1] <= r1[1] ||
			r2[0] <= r1[0] && r1[1] <= r2[1] {
			res++
		}
	}
	return fmt.Sprintf("%d", res)
}

func part2(input string) string {
	res := 0
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		r1, r2 := parseRanges(line)

		if r2[0] <= r1[1] && r1[0] <= r2[1] {
			res++
		}
	}
	return fmt.Sprintf("%d", res)
}
