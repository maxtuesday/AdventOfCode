package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day05 struct {
}

func (d Day05) Part1(input string) string {
	tokens := strings.Split(input, "\n")
	jumps := make([]int, len(tokens))
	for i := range tokens {
		jump, err := strconv.Atoi(tokens[i])
		if err != nil {
			panic(err)
		}
		jumps[i] = jump
	}

	// simulate jumps
	idx := 0
	steps := 0
	for 0 <= idx && idx < len(jumps) {
		// figure out next jump
		nextIdx := idx + jumps[idx]
		jumps[idx]++
		idx = nextIdx
		steps++
	}

	return fmt.Sprintf("%d", steps)
}

func (d Day05) Part2(input string) string {
	return ""
}
