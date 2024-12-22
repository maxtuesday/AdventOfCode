package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
)

func main() {
	file, err := os.Open("../../input/day10.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	scores := []int{}
	for scanner.Scan() {
		line := scanner.Text()

		open := GetOpenBrackets(line)
		if len(open) != 0 {
			closers := CloseBrackets(open)
			scores = append(scores, ScoreClosers(closers))
		}
	}

	// Sort scores
	sort.Ints(scores)
	// Get middle score

	fmt.Println("Result:", scores[len(scores)/2])
}

func GetVal(bracket rune) int {
	var val int
	switch bracket {
	case ')':
		val = 1
	case ']':
		val = 2
	case '}':
		val = 3
	case '>':
		val = 4
	}
	return val
}

func ScoreClosers(closers []rune) int {
	sum := 0
	for _, c := range closers {
		sum *= 5
		sum += GetVal(c)
	}
	return sum
}

func GetOpenBrackets(brackets string) []rune {
	open := []rune{}
	for _, b := range brackets {
		switch b {
		case '(', '[', '{', '<':
			open = append(open, b)
		default:
			// Take the last element from [open]
			// check if it is the correct matching character
			last := open[len(open)-1]
			checkStr := string(last) + string(b)
			switch checkStr {
			case "()", "[]", "{}", "<>":
				// Correct match, remove last from open
				open = open[:len(open)-1]
			default:
				// Corrupt Line. Discard.
				return []rune{}
			}
		}
	}

	return open
}

func CloseBrackets(open []rune) []rune {
	closingBrackets := []rune{}
	for i := len(open) - 1; i >= 0; i-- {
		var closer rune
		switch open[i] {
		case '(':
			closer = ')'
		case '[':
			closer = ']'
		case '{':
			closer = '}'
		case '<':
			closer = '>'
		}
		closingBrackets = append(closingBrackets, closer)
	}
	return closingBrackets
}

func DumpRunes(r []rune) {
	for i := range r {
		fmt.Print(string(r[i]))
	}
	fmt.Println()
}
