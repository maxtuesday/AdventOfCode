package solution

import (
	"fmt"
	"math"
	"strconv"
	"strings"
)

type Day02 struct {
}

func (d Day02) Part1(input string) string {
	sum := 0
	for _, line := range strings.Split(input, "\n") {
		min := math.MaxInt
		max := math.MinInt
		for _, num := range strings.Fields(line) {
			n, err := strconv.Atoi(num)
			if err != nil {
				panic(err)
			}

			if n < min {
				min = n
			}
			if n > max {
				max = n
			}
		}
		sum += max - min
	}
	return fmt.Sprintf("%d", sum)
}

func (d Day02) Part2(input string) string {
	return ""
}
