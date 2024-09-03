package solution

import (
	"fmt"
	"strings"
)

type Day11 struct {
}

func reduceBacksteps(f map[string]int, s1, s2 string) {
	m := min(f[s1], f[s2])
	f[s1] -= m
	f[s2] -= m
}

func reduceTwoStepOps(f map[string]int, s1, s2, target string) {
	m := min(f[s1], f[s2])
	f[target] += m
	f[s1] -= m
	f[s2] -= m
}

func (d Day11) Part1(input string) string {
	f := map[string]int{}

	steps := strings.Split(input, ",")
	for _, step := range steps {
		f[step]++
	}

	// reduce freqs based on reductions
	// n,s   -> 0
	reduceBacksteps(f, "n", "s")
	// ne,sw -> 0
	reduceBacksteps(f, "ne", "sw")
	// nw,se -> 0
	reduceBacksteps(f, "nw", "se")

	// n,se  -> ne
	reduceTwoStepOps(f, "n", "se", "ne")
	// n,sw  -> nw
	reduceTwoStepOps(f, "n", "sw", "nw")
	// s,ne  -> se
	reduceTwoStepOps(f, "s", "ne", "se")
	// s,nw  -> sw
	reduceTwoStepOps(f, "s", "nw", "sw")
	// ne,nw -> n
	reduceTwoStepOps(f, "ne", "nw", "n")
	// se,sw -> s
	reduceTwoStepOps(f, "se", "sw", "s")

	total := 0
	for _, v := range f {
		total += v
	}

	return fmt.Sprintf("%d", total)
}

func (d Day11) Part2(input string) string {
	return ""
}
