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
	b, _ := os.ReadFile("../../input/03/input.txt")
	return string(b)
}

func part1(input string) string {
	res := 0
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		a := line[:len(line)/2]
		b := line[len(line)/2:]

		var item byte = 0

		setA := map[byte]int{}
		for i := range a {
			setA[a[i]]++
		}
		for i := range b {
			if _, ok := setA[b[i]]; ok {
				item = b[i]
				break
			}
		}

		if item >= 'a' && item <= 'z' {
			res += int(item-'a') + 1
		} else {
			res += int(item-'A') + 27
		}
	}
	return fmt.Sprintf("%d", res)
}

func part2(input string) string {
	res := 0
	lines := strings.Split(input, "\n")

	groups := [][]string{}
	for i := 0; i < len(lines); i += 3 {
		groups = append(groups, lines[i:i+3])
	}

	for _, group := range groups {
		setA := map[byte]int{}
		setB := map[byte]int{}

		for j := 0; j < 2; j++ {
			sack := group[j]
			for i := range sack {
				if j == 0 {
					setA[sack[i]]++
				} else {
					setB[sack[i]]++
				}
			}
		}

		var item byte = 0
		sack := group[2]
		for i := range sack {
			_, okA := setA[sack[i]]
			_, okB := setB[sack[i]]
			if okA && okB {
				item = sack[i]
				break
			}
		}

		if item >= 'a' && item <= 'z' {
			res += int(item-'a') + 1
		} else {
			res += int(item-'A') + 27
		}
	}
	return fmt.Sprintf("%d", res)
}
