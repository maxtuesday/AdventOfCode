package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day10 struct {
	sizeOverride int
}

func reverse(arr []int, i, j int) {
	size := len(arr)
	for i < j {
		arr[i%size], arr[j%size] = arr[j%size], arr[i%size]
		i++
		j--
	}
}

func (d Day10) Part1(input string) string {
	lengthStrs := strings.Fields(strings.ReplaceAll(input, ",", " "))
	lengths := make([]int, len(lengthStrs))
	for i := range lengthStrs {
		lengths[i], _ = strconv.Atoi(lengthStrs[i])
	}

	size := 256
	if d.sizeOverride != 0 {
		size = d.sizeOverride
	}

	list := make([]int, size)
	for i := range list {
		list[i] = i
	}

	index := 0
	skip := 0
	for _, length := range lengths {
		reverse(list, index, index+length-1)
		index += length + skip
		skip++
	}

	// multiply the first two numbers
	return fmt.Sprintf("%d", list[0]*list[1])
}

func (d Day10) Part2(input string) string {
	return ""
}
