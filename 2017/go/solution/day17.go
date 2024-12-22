package solution

import (
	"fmt"
	"strconv"
)

type Day17 struct {
}

func (d Day17) Part1(input string) string {
	step, _ := strconv.Atoi(input)

	index := 0
	buffer := []int{0}
	for i := 1; i <= 2017; i++ {
		index = ((index + step) % i) + 1
		buffer = append(buffer[:index], buffer[index-1:]...)
		buffer[index] = i // insert after
	}

	return fmt.Sprintf("%d", buffer[(index+1)%len(buffer)])
}

func (d Day17) Part2(input string) string {
	step, _ := strconv.Atoi(input)

	index := 0
	nextToZero := 0
	for i := 1; i <= 50_000_000; i++ {
		index = ((index + step) % i) + 1
		if index == 1 {
			nextToZero = i
		}
	}

	return fmt.Sprintf("%d", nextToZero)
}
