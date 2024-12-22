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
	count := 0

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		words := strings.Fields(line)
		if isValidV2(words) {
			count++
		}
	}

	return fmt.Sprintf("%d", count)
}

func isValidV2(words []string) bool {
	seen := map[[26]byte]struct{}{}
	for _, word := range words {
		key := makeKey(word)
		if _, ok := seen[key]; ok {
			return false
		}
		seen[key] = struct{}{}
	}
	return true
}

func makeKey(word string) [26]byte {
	key := [26]byte{}
	for i := range word {
		key[int(word[i]-'a')]++
	}
	return key
}
