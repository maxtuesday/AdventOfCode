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
	b, _ := os.ReadFile("../../input/06/input.txt")
	return string(b)
}

func part1(input string) string {
	lines := strings.Split(input, "\n")
	for _, packet := range lines {
		for i := 0; i < len(packet)-4; i++ {
			window := packet[i : i+4]
			if unique(window) {
				return fmt.Sprintf("%d", i+4)
			}
		}
	}
	return ""
}

func part2(input string) string {
	lines := strings.Split(input, "\n")
	for _, packet := range lines {
		for i := 0; i < len(packet)-14; i++ {
			window := packet[i : i+14]
			if unique(window) {
				return fmt.Sprintf("%d", i+14)
			}
		}
	}
	return ""
}

func unique(str string) bool {
	set := map[byte]int{}
	for i := range str {
		if _, ok := set[str[i]]; ok {
			return false
		}
		set[str[i]]++
	}
	return true
}
