package solution

import "fmt"

type Day01 struct {
}

func (d Day01) Part1(input string) string {
	// If the next digit matches the current digit, add it to the resulting sum
	// List is circular, so the last digit should check if it matches the first digit

	sum := 0
	for i := 0; i < len(input)-1; i++ {
		current := input[i]
		next := input[i+1]
		if current == next {
			sum += int(current - '0')
		}
	}

	// check last digit as well
	if input[0] == input[len(input)-1] {
		sum += int(input[0] - '0')
	}

	return fmt.Sprintf("%d", sum)
}

func (d Day01) Part2(input string) string {
	sum := 0
	step := len(input) / 2
	for i := 0; i < step; i++ {
		current := input[i]
		next := input[i+step]
		if current == next {
			sum += int(current - '0')
		}
	}
	return fmt.Sprintf("%d", sum*2)
}
