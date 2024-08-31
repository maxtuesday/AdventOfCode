package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day10 struct {
	sizeOverride int
}

func (d Day10) reverse(arr []int, i, j int) {
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
		d.reverse(list, index, index+length-1)
		index += length + skip
		skip++
	}

	// multiply the first two numbers
	return fmt.Sprintf("%d", list[0]*list[1])
}

func (d Day10) Part2(input string) string {
	// convert characters into ASCII lengths
	lengths := []int{}
	for i := range input {
		lengths = append(lengths, int(input[i]))
	}
	lengths = append(lengths, []int{17, 31, 73, 47, 23}...)

	list := make([]int, 256)
	for i := range list {
		list[i] = i
	}

	index := 0
	skip := 0
	for i := 0; i < 64; i++ {
		for _, length := range lengths {
			d.reverse(list, index, index+length-1)
			index += length + skip
			skip++
		}
	}

	blocks := []int{}
	for i := 0; i < 16; i++ {
		// XOR elements
		offset := i * 16
		result := list[offset]
		for j := 1; j < 16; j++ {
			result ^= list[j+offset]
		}
		blocks = append(blocks, result)
	}

	result := ""
	for _, block := range blocks {
		result += fmt.Sprintf("%02x", block)
	}
	return result
}
