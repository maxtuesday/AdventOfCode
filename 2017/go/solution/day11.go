package solution

import (
	"fmt"
	"strings"
)

type Day11 struct {
}

func reduce(f map[string]int, s1, s2 string) int {
	m := min(f[s1], f[s2])
	f[s1] -= m
	f[s2] -= m
	return m
}

func reduceBacksteps(f map[string]int, s1, s2 string) {
	reduce(f, s1, s2)
}

func reduceTwoStepOps(f map[string]int, s1, s2, target string) {
	f[target] += reduce(f, s1, s2)
}

func getStepsAwayFromStart(f map[string]int) int {
	// reduce freqs based on reductions
	backsteps := [][]string{
		{"n", "s"},
		{"ne", "sw"},
		{"nw", "se"},
	}
	for _, backstep := range backsteps {
		reduceBacksteps(f, backstep[0], backstep[1])
	}

	twoStepOps := [][]string{
		{"n", "se", "ne"},
		{"n", "sw", "nw"},
		{"s", "ne", "se"},
		{"s", "nw", "sw"},
		{"ne", "nw", "n"},
		{"se", "sw", "s"},
	}
	for _, op := range twoStepOps {
		reduceTwoStepOps(f, op[0], op[1], op[2])
	}
	total := 0
	for _, v := range f {
		total += v
	}
	return total
}

func (d Day11) Part1(input string) string {
	f := map[string]int{}

	steps := strings.Split(input, ",")
	for _, step := range steps {
		f[step]++
	}

	total := getStepsAwayFromStart(copyMap(f))

	return fmt.Sprintf("%d", total)
}

func (d Day11) Part2(input string) string {
	f := map[string]int{}

	m := 0
	steps := strings.Split(input, ",")
	for _, step := range steps {
		f[step]++
		total := getStepsAwayFromStart(copyMap(f))
		if total > m {
			m = total
		}
	}

	return fmt.Sprintf("%d", m)
}
