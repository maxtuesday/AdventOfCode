package solution

import (
	"fmt"
	"strconv"
)

type Day17 struct {
}

func (d Day17) Part1(input string) string {
	steps, _ := strconv.Atoi(input)

	index := 0
	buffer := []int{0}
	for i := 1; i <= 2017; i++ {
		index += steps
		index %= len(buffer)
		buffer = append(buffer[:index+1], buffer[index:]...)
		buffer[index+1] = i // insert after
		index += 1
	}

	return fmt.Sprintf("%d", buffer[(index+1)%len(buffer)])
}

func (d Day17) Part2(input string) string {
	return ""
}
