package solution

import (
	"fmt"
	"regexp"
	"strings"
)

type Day19 struct {
}

func (d Day19) parse(input string) [][]string {
	diagram := [][]string{}

	lines := strings.Split(input, "\n")
	for _, line := range lines {
		tokens := strings.Split(line, "")
		diagram = append(diagram, tokens)
	}
	return diagram
}

type Pos struct {
	r int
	c int
}

func findStartingPosition(diagram [][]string) Pos {
	for i := range diagram[0] {
		if diagram[0][i] == "|" {
			return Pos{r: 0, c: i}
		}
	}
	panic("starting position not found")
}

func isOOB(pos Pos, diagram [][]string) bool {
	return pos.r < 0 || pos.r >= len(diagram) || pos.c < 0 || pos.c >= len(diagram[0])
}

func isLetter(a string) bool {
	r := regexp.MustCompile("[a-zA-Z]")
	return r.MatchString(a)
}

func (d Day19) traverse(diagram [][]string) ([]string, int) {
	// find starting location on first line
	p := findStartingPosition(diagram)
	dir := "south"

	letters := []string{}
	steps := 0
	for {
		// navigate until we find a +, then change direction
		// if we come across a letter, then we should collect it
		// if we come across an "space", then we should break

		if isOOB(p, diagram) {
			break
		}

		token := diagram[p.r][p.c]
		if token == " " {
			// reached end of diagram path
			break
		}

		steps++
		if isLetter(token) {
			// letter
			letters = append(letters, token)
		}

		if token == "+" {
			// change direction
			switch dir {
			case "north", "south":
				// check which side has a "-"
				check := Pos{
					r: p.r,
					c: p.c + 1,
				}
				if !isOOB(check, diagram) && diagram[p.r][p.c+1] == "-" || isLetter(diagram[p.r][p.c+1]) {
					dir = "east"
				} else {
					dir = "west"
				}
			case "east", "west":
				// check which side has a "|"
				check := Pos{
					r: p.r + 1,
					c: p.c,
				}
				if !isOOB(check, diagram) && diagram[p.r+1][p.c] == "|" || isLetter(diagram[p.r+1][p.c]) {
					dir = "south"
				} else {
					dir = "north"
				}
			}
		}

		switch dir {
		case "north":
			p.r--
		case "south":
			p.r++
		case "east":
			p.c++
		case "west":
			p.c--
		}
	}
	return letters, steps
}

func (d Day19) Part1(input string) string {
	diagram := d.parse(input)
	letters, _ := d.traverse(diagram)
	return strings.Join(letters, "")
}

func (d Day19) Part2(input string) string {
	diagram := d.parse(input)
	_, steps := d.traverse(diagram)
	return fmt.Sprintf("%d", steps)
}
