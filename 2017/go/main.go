package main

import (
	"advent/solution"
	"fmt"
	"os"
	"strconv"
	"time"
)

type Solution interface {
	Part1(input string) string
	Part2(input string) string
}

var solutions = []Solution{
	solution.Day01{},
	solution.Day02{},
	solution.Day03{},
	solution.Day04{},
	solution.Day05{},
	solution.Day06{},
	solution.Day07{},
	solution.Day08{},
	solution.Day09{},
	solution.Day10{},
	solution.Day11{},
	solution.Day12{},
	solution.Day13{},
	solution.Day14{},
	solution.Day15{},
}

func readInput(day int) string {
	content, err := os.ReadFile(fmt.Sprintf("../input/day%02d.txt", day))
	if err != nil {
		panic(err)
	}
	return string(content)
}

func Solve(day int) {
	fmt.Printf("Day %02d:\n", day)
	input := readInput(day)
	sol := solutions[day-1]

	now := time.Now()
	fmt.Printf("Part 1: %-15s%s\n", sol.Part1(input), time.Since(now))
	now = time.Now()
	fmt.Printf("Part 2: %-15s%s\n", sol.Part2(input), time.Since(now))
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

	if day < 1 || day > len(solutions) {
		panic("day is out-of-bounds")
	}

	Solve(day)
}
