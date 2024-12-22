package solution

import (
	"fmt"
	"strconv"
	"strings"
)

type Day16 struct {
	overrideProgram string
}

func (d Day16) SearchChar(b []byte, needle byte) int {
	for i := range b {
		if b[i] == needle {
			return i
		}
	}
	panic(fmt.Sprintf("needle not found: %c", needle))
}

func (d Day16) Dance(p string, instructions []string) string {
	programs := []byte(p)
	for _, instruction := range instructions {
		switch {
		case strings.HasPrefix(instruction, "s"):
			// spin
			count, _ := strconv.Atoi(instruction[1:])
			index := len(programs) - count
			programs = append(programs[index:], programs[:index]...)

		case strings.HasPrefix(instruction, "x"):
			// swap positions
			tokens := strings.Split(instruction[1:], "/")
			a, _ := strconv.Atoi(tokens[0])
			b, _ := strconv.Atoi(tokens[1])
			programs[a], programs[b] = programs[b], programs[a]

		case strings.HasPrefix(instruction, "p"):
			// swap letters
			// find each letter
			tokens := strings.Split(instruction[1:], "/")
			a := d.SearchChar(programs, tokens[0][0])
			b := d.SearchChar(programs, tokens[1][0])
			programs[a], programs[b] = programs[b], programs[a]

		default:
			panic(fmt.Sprintf("unknown instruction: %s", instruction))
		}
	}
	return string(programs)
}

func (d Day16) Part1(input string) string {
	p := "abcdefghijklmnop"
	if d.overrideProgram != "" {
		p = d.overrideProgram
	}
	instructions := strings.Split(input, ",")
	return d.Dance(p, instructions)
}

func (d Day16) CycleLength(p string, instructions []string) int {
	seen := map[string]struct{}{}

	i := 0
	for {
		p = d.Dance(p, instructions)
		if _, ok := seen[p]; !ok {
			seen[p] = struct{}{}
		} else {
			return i
		}
		i++
	}
}

func (d Day16) Part2(input string) string {
	p := "abcdefghijklmnop"
	instructions := strings.Split(input, ",")

	// find cycle length
	cycleLength := d.CycleLength(p, instructions)

	// we would like to dance 1_000_000_000 times.
	remainder := 1_000_000_000 % cycleLength
	for i := 0; i < remainder; i++ {
		p = d.Dance(p, instructions)
	}

	return p
}
