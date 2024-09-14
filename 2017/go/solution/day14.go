package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day14 struct {
}

func (d Day14) Part1(input string) string {
	hashes := make([]string, 128)
	for i := 0; i < len(hashes); i++ {
		hashes[i] = KnotHash(fmt.Sprintf("%s-%d", input, i))
	}

	// convert the hex hash into 128 bit binary
	rows := make([]string, len(hashes))
	for i := range rows {
		row := ""
		for j := range hashes[i] {
			ch := hashes[i][j]
			n, _ := strconv.ParseInt(string(ch), 16, 0)
			bin := fmt.Sprintf("%04b", n)
			row += bin
		}
		rows = append(rows, row)
	}

	// count how many used spots are in each row
	used := 0
	for _, row := range rows {
		used += strings.Count(row, "1")
	}

	return fmt.Sprintf("%d", used)
}

func (d Day14) Part2(input string) string {
	return ""
}
