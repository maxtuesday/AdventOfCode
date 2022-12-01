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
	b, err := os.ReadFile("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	data := string(b)
	dataSlice := strings.Split(data, "\n")

	part1(dataSlice)
	part2(dataSlice)
}

func part1(dataSlice []string) {
	fmt.Printf("Part 1: ")
	max := 0
	curSum := 0
	for _, line := range dataSlice {
		if line == "" {
			if curSum > max {
				max = curSum
			}
			curSum = 0
			continue
		}
		num, _ := strconv.Atoi(line)
		curSum += num
	}

	fmt.Printf("Ans: %d\n", max)
}

func part2(dataSlice []string) {
	fmt.Printf("Part 2: ")
	cals := []int{}
	curSum := 0
	for _, line := range dataSlice {
		if line == "" {
			cals = append(cals, curSum)
			curSum = 0
			continue
		}
		num, _ := strconv.Atoi(line)
		curSum += num
	}
	sort.Ints(cals)
	l := len(cals)
	ans := cals[l-1] + cals[l-2] + cals[l-3]
	fmt.Printf("Ans: %d\n", ans)
}
