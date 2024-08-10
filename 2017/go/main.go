package main

import (
	"advent/solution"
	"fmt"
	"os"
	"strconv"
)

type Solution interface {
	Part1(input string) string
	Part2(input string) string
}

func readInput(day int) string {
	content, err := os.ReadFile(fmt.Sprintf("../input/day%02d.txt", day))
	if err != nil {
		panic(err)
	}
	return string(content)
}

func Solve(sol Solution, input string) {
	fmt.Printf("Part 1: %s\n", sol.Part1(input))
	fmt.Printf("Part 2: %s\n", sol.Part2(input))
}

func main() {
	if len(os.Args) < 2 {
		panic("require day as argument (number)")
	}

	dayStr := os.Args[1]
	day, err := strconv.Atoi(dayStr)
	if err != nil {
		panic(err)
	}

	solutions := []Solution{
		solution.Day01{},
		solution.Day02{},
		solution.Day03{},
		solution.Day04{},
		solution.Day05{},
		solution.Day06{},
		solution.Day07{},
	}

	if day < 1 || day > len(solutions) {
		panic("day is out-of-bounds")
	}

	Solve(solutions[day-1], readInput(day))
}
