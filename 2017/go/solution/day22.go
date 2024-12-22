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

const (
	Clean    = 0
	Weakened = 1
	Infected = 2
	Flagged  = 3
)

type NodePart2 struct {
	pos   Position
	state int
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

func (d Day22) parsePart2(input string) map[Position]*NodePart2 {
	lines := strings.Split(input, "\n")
	if len(lines)%2 == 0 || len(lines[0])%2 == 0 {
		panic("there is no center location")
	}

	nodes := map[Position]*NodePart2{}

	centerY := len(lines) / 2
	centerX := len(lines[0]) / 2
	for y, line := range lines {
		for x := range line {
			pos := Position{
				x: x - centerX,
				y: centerY - y,
			}
			state := Clean
			if line[x] == '#' {
				state = Infected
			}
			nodes[pos] = &NodePart2{
				pos:   pos,
				state: state,
			}
		}
	}

	return nodes
}

func (d Day22) stepPart2(vc *VirusCarrier, nodes map[Position]*NodePart2) {
	var node *NodePart2
	if n, ok := nodes[vc.pos]; ok {
		node = n
	} else {
		node = &NodePart2{
			pos:   vc.pos,
			state: Clean,
		}
		nodes[vc.pos] = node
	}

	// Movement:
	// - Clean    -> Left
	// - Weakened -> No turn
	// - Infected -> Right
	// - Flagged  -> Reverse

	// Node update:
	// - Clean    -> Weakened
	// - Weakened -> Infected
	// - Infected -> Flagged
	// - Flagged  -> Clean

	switch node.state {
	case Clean:
		node.state = Weakened
		vc.Turn(LEFT)
	case Weakened:
		node.state = Infected
		vc.infectionCount++
	case Infected:
		node.state = Flagged
		vc.Turn(RIGHT)
	case Flagged:
		node.state = Clean
		vc.Turn(RIGHT)
		vc.Turn(RIGHT)
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

func (d Day22) DisplayGridPart2(vc *VirusCarrier, nodes map[Position]*NodePart2) {
	for y := 4; y >= -4; y-- {
		for x := -4; x <= 4; x++ {
			pos := Position{x, y}
			state := "."
			if node, ok := nodes[pos]; ok {
				switch node.state {
				case Clean:
				case Weakened:
					state = "W"
				case Infected:
					state = "#"
				case Flagged:
					state = "F"
				}
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
	nodes := d.parsePart2(input)
	vc := &VirusCarrier{
		pos:       Position{0, 0},
		direction: UP,
	}

	for i := 0; i < 10_000_000; i++ {
		d.stepPart2(vc, nodes)
	}

	return fmt.Sprintf("%d", vc.infectionCount)
}
