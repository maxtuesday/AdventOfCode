package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
)

func main() {
	file, err := os.Open("../../input/day10.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	illegalChars := []rune{}
	for scanner.Scan() {
		line := scanner.Text()

		// For each line, check for first invalid character
		if b, ok := CheckBrackets(line); !ok {
			illegalChars = append(illegalChars, b)
		}
	}

	sum := 0
	for _, c := range illegalChars {
		switch c {
		case ')':
			sum += 3
		case ']':
			sum += 57
		case '}':
			sum += 1197
		case '>':
			sum += 25137
		}
	}

	fmt.Println("Result:", sum)

}

func CheckBrackets(brackets string) (rune, bool) {
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
				// Oh no, this is not the closing character
				// This should be the first illegal character
				return b, false
			}
		}
	}

	return rune(0), true
}

func DumpRunes(r []rune) {
	for i := range r {
		fmt.Print(string(r[i]))
	}
	fmt.Println()
}
