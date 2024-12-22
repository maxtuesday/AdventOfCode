package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

const START = "start"
const END = "end"

const COUNT_MAX = 50

var COUNT = 0

var paths = [][]string{}

type Visited struct {
	v map[string]int
}

func main() {
	content, err := os.ReadFile("../../input/day12.txt")
	if err != nil {
		log.Fatal(err)
	}

	connections := strings.Split(string(content), "\n")
	connections = connections[:len(connections)-1]

	graph := make(map[string][]string)

	for _, connection := range connections {
		// fmt.Println(connection)
		caves := strings.Split(connection, "-")
		graph[caves[0]] = append(graph[caves[0]], caves[1])
		graph[caves[1]] = append(graph[caves[1]], caves[0])
	}

	Dump(graph)

	fmt.Println()

	for _, start := range graph[START] {
		ExplorePath(start, graph, []string{})
	}

	fmt.Println("Num Paths:", len(paths))
}

func ExplorePath(curr string, graph map[string][]string, path []string) []string {

	path = append(path, curr)

	// Is curr `end`
	if curr == END {
		return path
	}

	// Find which spots are left to explore
	next := []string{}
	possible := graph[curr]
	for _, p := range possible {
		if p != START {
			found := false
			for _, known := range path {
				if known == p {
					found = true
				}
			}
			if !found {
				next = append(next, p)
			} else {
				if strings.ToLower(p) != p {
					next = append(next, p)
				}
			}
		}
	}

	if len(next) == 0 {
		return nil
	}

	for _, n := range next {
		finishedPath := ExplorePath(n, graph, path)
		if finishedPath != nil {
			paths = append(paths, finishedPath)
		}
	}

	return nil
}

func Dump(graph map[string][]string) {
	for k, v := range graph {
		fmt.Printf("%s\t->\t%v\n", k, v)
	}
}
