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
	b, _ := os.ReadFile("../../inputs/07/input.txt")
	// b, _ := os.ReadFile("../../inputs/07/small_input.txt")
	return string(b)
}

func popDir(path string) string {
	pathTokens := strings.Split(path, "/")
	return strings.Join(pathTokens[:len(pathTokens)-1], "/")
}

func pushDir(path, dir string) string {
	path += "/" + dir
	if path[:2] == "//" { // clean up path
		path = path[1:]
	}
	return path
}

// Has some bug with the large input where the root dir
// does not get the correct total directory size
func parseInput(input string) map[string]int {
	/*
		Pattern:
		cd into dir
		ls
		if no more dir, pop up until next dir
	*/

	// skip the first cd
	lines := strings.Split(input, "\n")[1:]
	fs := map[string]int{}
	sum := 0
	path := "/"
	for _, line := range lines {
		tokens := strings.Split(line, " ")
		if tokens[0] == "$" { // command
			cmd := tokens[1]
			if cmd == "cd" {
				dir := tokens[2]
				switch dir {
				case "..": // pop up one dir
					// start "flattening" dir sizes
					fs[path] += sum
					sum = fs[path]
					path = popDir(path)
					if path == "" {
						path = "/"
					}
				default: // move to this directory
					// when we cd into a dir, we have listed all files at the current path
					// add current list of file sizes to current path
					fs[path] += sum
					// reset sum since we will be at a different level
					path = pushDir(path, dir)
				}
			} else {
				// list operation, get to baseline for current directory
				sum = 0
			}
		} else { // file or dir
			if size, err := strconv.Atoi(tokens[0]); err == nil {
				// file
				sum += size
			}
		}
	}

	// we are done. Might be at the bottom of a directory
	// let's bubble up and add up this sum to parent directories
	// This is probably where that bug metioned above is
	for path != "" {
		fs[path] += sum
		path = popDir(path)
	}
	fs["/"] += sum
	return fs
}

func getTotalSize(input string) int {
	sum := 0
	lines := strings.Split(input, "\n")
	for _, line := range lines {
		tokens := strings.Split(line, " ")
		if size, err := strconv.Atoi(tokens[0]); err == nil {
			sum += size
		}
	}
	return sum
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
	totalUsed := getTotalSize(input)
	fs := parseInput(input)

	TOTAL_DISK_SPACE := 70000000
	UNUSED_REQUIRED := 30000000
	remainingSpace := TOTAL_DISK_SPACE - totalUsed
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
