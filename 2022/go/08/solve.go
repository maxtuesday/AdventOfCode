package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	input := readInput()
	fmt.Printf("Part 1: %s\n", part1(input))
	fmt.Printf("Part 2: %s\n", part2(input))
}

func readInput() string {
	b, _ := os.ReadFile("../../input/08/input.txt")
	return string(b)
}

func parseTrees(input string) [][]int {
	lines := strings.Split(input, "\n")
	trees := make([][]int, len(lines))
	for r, line := range lines {
		trees[r] = make([]int, len(line))
		for i := range line {
			trees[r][i] = int(line[i] - '0')
		}
	}
	return trees
}

func part1(input string) string {
	trees := parseTrees(input)
	visible := map[[2]int]struct{}{}
	for r := range trees {
		maxColTreeHeight := -1
		for c := range trees[r] {
			if trees[r][c] > maxColTreeHeight {
				maxColTreeHeight = trees[r][c]
				visible[[2]int{r, c}] = struct{}{}
			}
		}
		maxColTreeHeight = -1
		for c := len(trees[r]) - 1; c >= 0; c-- {
			if trees[r][c] > maxColTreeHeight {
				maxColTreeHeight = trees[r][c]
				visible[[2]int{r, c}] = struct{}{}
			}
		}
	}

	for c := range trees[0] {
		maxColTreeHeight := -1
		for r := range trees {
			if trees[r][c] > maxColTreeHeight {
				maxColTreeHeight = trees[r][c]
				visible[[2]int{r, c}] = struct{}{}
			}
		}

		maxColTreeHeight = -1
		for r := len(trees) - 1; r >= 0; r-- {
			if trees[r][c] > maxColTreeHeight {
				maxColTreeHeight = trees[r][c]
				visible[[2]int{r, c}] = struct{}{}
			}
		}
	}
	return fmt.Sprintf("%d", len(visible))
}

func part2(input string) string {
	trees := parseTrees(input)
	res := -1
	for r := 1; r < len(trees)-1; r++ {
		for c := 1; c < len(trees[r])-1; c++ {
			res = max(res, getScore(r, c, trees))
		}
	}
	return fmt.Sprintf("%d", res)
}

func getScore(r, c int, trees [][]int) int {
	tree := trees[r][c]
	// left
	left := 0
	for i := c - 1; i >= 0; i-- {
		left++
		if trees[r][i] >= tree {
			break
		}
	}
	// right
	right := 0
	for i := c + 1; i < len(trees[0]); i++ {
		right++
		if trees[r][i] >= tree {
			break
		}
	}
	// up
	up := 0
	for i := r - 1; i >= 0; i-- {
		up++
		if trees[i][c] >= tree {
			break
		}
	}
	// down
	down := 0
	for i := r + 1; i < len(trees); i++ {
		down++
		if trees[i][c] >= tree {
			break
		}
	}

	return left * right * up * down
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
