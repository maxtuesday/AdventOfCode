package solution

import (
	"fmt"
	"strings"
)

type Day22 struct {
}

type Position struct {
	x, y int
}

type Node struct {
	pos      Position
	infected bool
}

type Direction [2]int

var (
	UP    = Direction{1, 0}
	DOWN  = Direction{-1, 0}
	LEFT  = Direction{0, -1}
	RIGHT = Direction{0, 1}
)

type VirusCarrier struct {
	pos            Position
	direction      Direction
	infectionCount int
}

func (vc *VirusCarrier) Turn(dir Direction) {
	switch vc.direction {
	case UP:
		vc.direction = dir
	case DOWN:
		switch dir {
		case LEFT:
			vc.direction = RIGHT
		case RIGHT:
			vc.direction = LEFT
		}
	case LEFT:
		switch dir {
		case LEFT:
			vc.direction = DOWN
		case RIGHT:
			vc.direction = UP
		}
	case RIGHT:
		switch dir {
		case LEFT:
			vc.direction = UP
		case RIGHT:
			vc.direction = DOWN
		}
	}
}

func (vc *VirusCarrier) MoveForward() {
	vc.pos.y += vc.direction[0]
	vc.pos.x += vc.direction[1]
}

func (d Day22) parse(input string) map[Position]*Node {
	lines := strings.Split(input, "\n")
	if len(lines)%2 == 0 || len(lines[0])%2 == 0 {
		panic("there is no center location")
	}

	nodes := map[Position]*Node{}

	centerY := len(lines) / 2
	centerX := len(lines[0]) / 2
	for y, line := range lines {
		for x := range line {
			pos := Position{
				x: x - centerX,
				y: centerY - y,
			}
			infected := false
			if line[x] == '#' {
				infected = true
			}
			nodes[pos] = &Node{
				pos:      pos,
				infected: infected,
			}
		}
	}

	return nodes
}

func (d Day22) step(vc *VirusCarrier, nodes map[Position]*Node) {
	// If current node is infected, turn right; otherwise turn left.
	// If current node is clean, it becomes infected; otherwise it becomes clean.
	if node, ok := nodes[vc.pos]; ok && node.infected {
		// node is infected
		// turn right
		vc.Turn(RIGHT)
		nodes[vc.pos].infected = false
	} else {
		// node is not infected (clean)
		// turn left
		vc.Turn(LEFT)
		if node != nil {
			nodes[vc.pos].infected = true
		} else {
			nodes[vc.pos] = &Node{
				pos:      vc.pos,
				infected: true,
			}
		}
		vc.infectionCount++
	}
	vc.MoveForward()
}

func (d Day22) DisplayGrid(vc *VirusCarrier, nodes map[Position]*Node) {
	for y := 4; y >= -4; y-- {
		for x := -4; x <= 4; x++ {
			pos := Position{x, y}
			state := "."
			if node, ok := nodes[pos]; ok && node.infected {
				state = "#"
			}

			if vc.pos == pos {
				fmt.Printf("[%s]", state)
			} else {
				fmt.Printf(" %s ", state)
			}
		}
		fmt.Println()
	}
	fmt.Println()
}

func (d Day22) Part1(input string) string {
	nodes := d.parse(input)
	vc := &VirusCarrier{
		pos:       Position{0, 0},
		direction: UP,
	}

	for i := 0; i < 10_000; i++ {
		d.step(vc, nodes)
	}

	return fmt.Sprintf("%d", vc.infectionCount)
}

func (d Day22) Part2(input string) string {
	return ""
}
