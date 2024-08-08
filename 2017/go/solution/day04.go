package solution

import (
	"fmt"
	"strings"
)

type Day04 struct {
}

func (d Day04) Part1(input string) string {
	count := 0

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		words := strings.Fields(line)
		if isValid(words) {
			count++
		}
	}

	return fmt.Sprintf("%d", count)
}

func isValid(words []string) bool {
	seen := map[string]struct{}{}
	for _, word := range words {
		if _, ok := seen[word]; ok {
			return false
		}
		seen[word] = struct{}{}
	}
	return true
}

func (d Day04) Part2(input string) string {
	return ""
}
