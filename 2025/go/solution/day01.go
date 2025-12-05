package solution

import (
	"log"
	"strconv"
	"strings"
)

type Day01 struct{}

func (d Day01) Part1(input string) string {
	// L -> ((100 - x) + cur) mod 100
	// R -> (x + cur) mod 100

	var (
		count = 0
		pos   = 50
	)

	lines := strings.SplitSeq(input, "\n")
	for line := range lines {
		// (L|R)(num)
		dir := line[0]
		num, _ := strconv.Atoi(line[1:])
		switch dir {
		case 'L':
			pos = 100 - num + pos
		case 'R':
			pos = num + pos
		default:
			log.Fatalf("unknown first character: %c\n", dir)
		}
		pos %= 100
		if pos == 0 {
			count += 1
		}
	}

	return strconv.Itoa(count)
}

func (d Day01) Part2(input string) string {
	// Count every time 0 is passed
	var (
		count = 0
		pos   = 50
	)

	lines := strings.SplitSeq(input, "\n")
	for line := range lines {
		dir := line[0]
		num, _ := strconv.Atoi(line[1:])
		switch dir {
		case 'L':
			// Don't go negative
			if pos == 0 {
				pos = 100
			}
			for _ = range num {
				pos -= 1
				if pos == 0 {
					count += 1
					pos = 100
				}
			}
		case 'R':
			// Don't go above 100
			if pos == 100 {
				pos = 0
			}
			for _ = range num {
				pos += 1
				if pos == 100 {
					count += 1
					pos = 0
				}
			}
		default:
			log.Fatalf("unknown first character: %c\n", dir)
		}
	}
	return strconv.Itoa(count)
}
