package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	MIN_H = -7
	MAX_H = 7
	MIN_W = -7
	MAX_W = 7
)

var START = []int{0, 0}

func main() {
	input := readInput()
	fmt.Printf("Part 1: %s\n", part1(input))
	fmt.Printf("Part 2: %s\n", part2(input))
}

func readInput() string {
	b, _ := os.ReadFile("../../input/09/input.txt")
	return string(b)
}

var dirs = map[string][]int{
	"U": {1, 0},
	"D": {-1, 0},
	"L": {0, -1},
	"R": {0, 1},
}

func getMagnitude(p1, p2 int) int {
	d := p1 - p2
	if -2 < d && d < 2 {
		return 0
	}
	if d > 0 {
		return -1
	} else {
		return 1
	}
}

func updateTail(head, tail []int) {
	dy := getMagnitude(head[0], tail[0])
	dx := getMagnitude(head[1], tail[1])
	if dy == 0 && dx == 0 {
		return
	}
	tail[0] = head[0] + dy
	tail[1] = head[1] + dx
}

func moveRope(input string, numKnots int) int {
	visited := map[[2]int]int{}
	knots := [][]int{}
	for i := 0; i < numKnots; i++ {
		knots = append(knots, []int{0, 0})
	}
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		move(line, knots, visited)
	}
	return len(visited)
}

func move(line string, knots [][]int, visited map[[2]int]int) {
	tokens := strings.Split(line, " ")
	dir := tokens[0]
	steps, _ := strconv.Atoi(tokens[1])
	change := dirs[dir]
	for i := 0; i < steps; i++ {
		knots[0][0] += change[0]
		knots[0][1] += change[1]
		// update tail
		for i := 1; i < len(knots); i++ {
			updateTail(knots[i-1], knots[i])
		}
		tail := knots[len(knots)-1]
		visited[[2]int{tail[0], tail[1]}]++
	}
}

func part1(input string) string {
	ans := moveRope(input, 2)
	return fmt.Sprintf("%d", ans)
}

func part2(input string) string {
	ans := moveRope(input, 10)
	return fmt.Sprintf("%d", ans)
}

/*
	VISUALIZATION HELPERS
*/

func printKnots(head []int, knots [][]int, minW, minH, maxW, maxH int) {
	fmt.Println()
	for i := maxH - 1; i >= minH; i-- {
		for j := minW; j < maxW; j++ {
			char := "."
			if START[0] == i && START[1] == j {
				char = "s"
			}
			for k := len(knots) - 1; k >= 0; k-- {
				if knots[k][0] == i && knots[k][1] == j {
					char = fmt.Sprintf("%d", k+1)
				}
			}
			if head[0] == i && head[1] == j {
				char = "H"
			}
			fmt.Printf("%s", char)
		}
		fmt.Printf("\n")
	}
	fmt.Println()
}

func printVisited(visited map[[2]int]int, minW, minH, maxW, maxH int) {
	fmt.Println()
	for i := maxH - 1; i >= minH; i-- {
		for j := minW; j < maxW; j++ {
			char := "."
			if _, ok := visited[[2]int{i, j}]; ok {
				char = "#"
			}
			if START[0] == i && START[1] == j {
				char = "s"
			}
			fmt.Printf("%s", char)
		}
		fmt.Printf("\n")
	}
	fmt.Println()
}
