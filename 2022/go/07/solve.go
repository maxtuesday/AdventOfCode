package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	input := readInput()
	fmt.Printf("Part 1: %s\n", part1(input))
	fmt.Printf("Part 2: %s\n", part2(input))
}

func readInput() string {
	b, _ := os.ReadFile("../../input/07/input.txt")
	return string(b)
}

func popDir(path string) string {
	pathTokens := strings.Split(path, "/")
	path = strings.Join(pathTokens[:len(pathTokens)-1], "/")
	if path == "" {
		path = "/"
	}
	return path
}

func pushDir(path, dir string) string {
	path += "/" + dir
	if path[:2] == "//" { // clean up path
		path = path[1:]
	}
	return path
}

func parseInput(input string) map[string]int {
	lines := strings.Split(input, "\n")[1:]
	fs := map[string]int{}
	path := "/"
	for _, line := range lines {
		tokens := strings.Split(line, " ")
		if tokens[0] == "$" { // command
			cmd := tokens[1]
			if cmd == "cd" {
				dir := tokens[2]
				switch dir {
				case "..":
					path = popDir(path)
				default:
					path = pushDir(path, dir)
				}
			}
		} else {
			if size, err := strconv.Atoi(tokens[0]); err == nil {
				fs[path] += size
			}
		}
	}

	originalFS := map[string]int{}
	for path, size := range fs {
		originalFS[path] = size
	}

	// Apply flattening of child dirs
	for path, size := range originalFS {
		for path != "/" {
			path = popDir(path)
			fs[path] += size
		}
	}

	return fs
}

func part1(input string) string {
	MAX := 100000
	res := 0
	fs := parseInput(input)
	for _, size := range fs {
		if size <= MAX {
			res += size
		}
	}
	return fmt.Sprintf("%d", res)
}

func part2(input string) string {
	TOTAL_DISK_SPACE := 70000000
	UNUSED_REQUIRED := 30000000

	fs := parseInput(input)
	remainingSpace := TOTAL_DISK_SPACE - fs["/"]
	minRequired := UNUSED_REQUIRED - remainingSpace

	sizes := []int{}
	for _, size := range fs {
		sizes = append(sizes, size)
	}
	sort.Ints(sizes)

	for _, size := range sizes {
		if minRequired < size {
			return fmt.Sprintf("%d", size)
		}
	}
	return "Not found"
}
