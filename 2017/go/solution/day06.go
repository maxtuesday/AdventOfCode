package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day06 struct {
}

func (d Day06) parse(input string) []int {
	tokens := strings.Fields(input)
	blocks := make([]int, len(tokens))
	for i := range tokens {
		block, err := strconv.Atoi(tokens[i])
		if err != nil {
			panic(err)
		}
		blocks[i] = block
	}
	return blocks
}

func redistribute(blocks []int) string {
	N := len(blocks)

	// find block with most
	max, maxIdx := blocks[0], 0
	for i := range blocks {
		if blocks[i] > max {
			max, maxIdx = blocks[i], i
		}
	}

	blocksToDist := blocks[maxIdx]
	blocks[maxIdx] = 0 // set to 0 since we are going to redist the value

	idx := (maxIdx + 1) % N
	for blocksToDist > 0 {
		blocks[idx]++
		blocksToDist--
		idx = (idx + 1) % N
	}

	return fmt.Sprintf("%v", blocks)
}

func (d Day06) Part1(input string) string {
	blocks := d.parse(input)

	seen := map[string]struct{}{}
	seen[fmt.Sprintf("%v", blocks)] = struct{}{}

	steps := 0

	for {
		key := redistribute(blocks)
		steps++
		if _, ok := seen[key]; ok {
			return fmt.Sprintf("%d", steps)
		}
		seen[key] = struct{}{}
	}
}

func (d Day06) Part2(input string) string {
	blocks := d.parse(input)

	seen := map[string]int{}
	seen[fmt.Sprintf("%v", blocks)] = 0

	steps := 0

	for {
		key := redistribute(blocks)
		steps++
		if v, ok := seen[key]; ok {
			return fmt.Sprintf("%d", steps-v)
		}
		seen[key] = steps
	}
}
