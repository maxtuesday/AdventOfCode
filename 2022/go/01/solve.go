package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	lines := readLines()

	part1(lines)
	part2(lines)
}

func readLines() []string {
	b, err := os.ReadFile("../../input/01/input.txt")
	if err != nil {
		log.Fatal(err)
	}

	data := string(b)
	return strings.Split(data, "\n")
}

func part1(lines []string) {
	sums := sum(lines)
	sort.Ints(sums)
	ans := sums[len(sums)-1]
	fmt.Printf("Part 1: %d\n", ans)
}

func part2(lines []string) {
	sums := sum(lines)
	sort.Ints(sums)
	l := len(sums)
	ans := sums[l-1] + sums[l-2] + sums[l-3]
	fmt.Printf("Part 2: %d\n", ans)
}

func sum(lines []string) []int {
	sum := 0
	sums := []int{}
	for _, line := range lines {
		if line == "" {
			sums = append(sums, sum)
			sum = 0
			continue
		}
		num, _ := strconv.Atoi(line)
		sum += num
	}
	return sums
}
