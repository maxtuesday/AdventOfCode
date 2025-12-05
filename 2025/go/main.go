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
	p1 := sol.Part1(input)
	fmt.Printf("[%10s] Part 1: %s\n", time.Since(now).Round(time.Microsecond), p1)
	now = time.Now()
	p2 := sol.Part2(input)
	fmt.Printf("[%10s] Part 2: %s\n", time.Since(now).Round(time.Microsecond), p2)
}

func main() {
	if len(os.Args) < 2 {
		for i := range solutions {
			Solve(i + 1)
		}
		return
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
