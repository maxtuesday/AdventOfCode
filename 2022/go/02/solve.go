package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	input := readInput()
	fmt.Printf("Part 1: %s\n", part1(input))
	fmt.Printf("Part 2: %s\n", part2(input))
}

func readInput() string {
	b, _ := os.ReadFile("../../input/02/input.txt")
	return string(b)
}

func part1(input string) string {
	res := 0
	lines := strings.Split(input, "\n")

	x := map[string]int{
		"A": 3,
		"B": 0,
		"C": 6,
	}

	y := map[string]int{
		"A": 6,
		"B": 3,
		"C": 0,
	}

	z := map[string]int{
		"A": 0,
		"B": 6,
		"C": 3,
	}

	for _, line := range lines {
		parts := strings.Split(line, " ")
		a := parts[0]
		b := parts[1]

		switch b {
		case "X":
			res += x[a] + 1
		case "Y":
			res += y[a] + 2
		case "Z":
			res += z[a] + 3
		}
	}
	return fmt.Sprintf("%d", res)
}

func part2(input string) string {
	res := 0
	lines := strings.Split(input, "\n")

	x := map[string]int{
		"A": 3,
		"B": 1,
		"C": 2,
	}

	y := map[string]int{
		"A": 1,
		"B": 2,
		"C": 3,
	}

	z := map[string]int{
		"A": 2,
		"B": 3,
		"C": 1,
	}

	for _, line := range lines {
		parts := strings.Split(line, " ")
		a := parts[0]
		b := parts[1]

		switch b {
		case "X": // lose
			res += x[a]
		case "Y": // tie
			res += y[a] + 3
		case "Z": // win
			res += z[a] + 6
		}
	}
	return fmt.Sprintf("%d", res)
}
