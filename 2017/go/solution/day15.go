package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day15 struct {
}

const (
	FactorA = 16807
	FactorB = 48271
	Div     = 2147483647 // 2^31 - 1
)

func getStartingValue(line string) int {
	tokens := strings.Fields(line)
	n, _ := strconv.Atoi(tokens[len(tokens)-1])
	return n
}

func (d Day15) Part1(input string) string {
	lines := strings.Split(input, "\n")
	a := getStartingValue(lines[0])
	b := getStartingValue(lines[1])

	count := 0
	for i := 0; i < 40_000_000; i++ {
		a = (a * FactorA) % Div
		b = (b * FactorB) % Div

		aa := (uint32(a) << 16) >> 16
		bb := (uint32(b) << 16) >> 16
		if aa == bb {
			count++
		}
	}

	return fmt.Sprintf("%d", count)
}

func (d Day15) Part2(input string) string {
	return ""
}
