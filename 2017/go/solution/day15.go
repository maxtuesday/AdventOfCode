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

func (d Day15) parse(input string) (int, int) {
	lines := strings.Split(input, "\n")
	a := getStartingValue(lines[0])
	b := getStartingValue(lines[1])
	return a, b
}

func (d Day15) Part1(input string) string {
	a, b := d.parse(input)

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

type Generator struct {
	n      int
	factor int
	mod    int
}

func (g *Generator) Next() int {
	for {
		g.n = (g.n * g.factor) % Div
		if g.n%g.mod == 0 {
			return g.n
		}
	}
}

// Can improve time with go routines
func (d Day15) Part2(input string) string {
	an, bn := d.parse(input)

	genA := Generator{
		n:      an,
		factor: FactorA,
		mod:    4,
	}
	genB := Generator{
		n:      bn,
		factor: FactorB,
		mod:    8,
	}

	count := 0
	for i := 0; i < 5_000_000; i++ {
		a := genA.Next()
		b := genB.Next()

		aa := (uint32(a) << 16) >> 16
		bb := (uint32(b) << 16) >> 16
		if aa == bb {
			count++
		}
	}

	return fmt.Sprintf("%d", count)
}
