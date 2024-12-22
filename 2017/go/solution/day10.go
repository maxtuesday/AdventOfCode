package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day10 struct {
	sizeOverride int
}

func reverseCyclical[T any](arr []T, i, j int) {
	size := len(arr)
	for i < j {
		arr[i%size], arr[j%size] = arr[j%size], arr[i%size]
		i++
		j--
	}
}

func InitKnotHashList(size int) []int {
	list := make([]int, size)
	for i := range list {
		list[i] = i
	}
	return list
}

func KnotHashRounds(list, lengths []int, rounds int) {
	index, skip := 0, 0
	for i := 0; i < rounds; i++ {
		for _, length := range lengths {
			reverseCyclical(list, index, index+length-1)
			index += length + skip
			skip++
		}
	}
}

func KnotHash(input string) string {
	// convert characters into ASCII lengths
	lengths := []int{}
	for i := range input {
		lengths = append(lengths, int(input[i]))
	}
	lengths = append(lengths, []int{17, 31, 73, 47, 23}...)

	list := InitKnotHashList(256)
	KnotHashRounds(list, lengths, 64)

	hex := ""
	for i := 0; i < 16; i++ {
		offset := i * 16
		block := list[offset]
		for j := 1; j < 16; j++ {
			block ^= list[j+offset]
		}
		hex += fmt.Sprintf("%02x", block)
	}
	return hex
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

	list := InitKnotHashList(size)
	KnotHashRounds(list, lengths, 1)

	return fmt.Sprintf("%d", list[0]*list[1])
}

func (d Day10) Part2(input string) string {
	return KnotHash(input)
}
